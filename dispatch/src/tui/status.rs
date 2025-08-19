use std::collections::BTreeSet;
use std::net::{IpAddr, SocketAddr};
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

use chrono::{DateTime, Local};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Cell, Paragraph, Row, Table};

use crate::github::Asset;
use crate::jobs::{Job, Jobs, State};

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

    let mut size = bytes;
    let mut index = 0;

    while size >= 1024 && index < UNITS.len() - 1 {
        size /= 1024;
        index += 1;
    }

    let size_f = (bytes as f64) / (1024_f64.powi(index as i32));
    format!("{:.1} {}", size_f, UNITS[index])
}

impl State {
    const fn emoji(&self) -> &'static str {
        match self {
            Self::Unassigned => "⏳",
            Self::Assigned(..) => "📌",
            Self::Downloading(..) => "📥",
            Self::Booting(..) => "⚡",
            Self::Reported(..) => "📝",
            Self::Failed(..) => "❌",
            Self::Finished(..) => "🏁",
        }
    }
}

impl Job {
    fn style(&self) -> Style {
        match self.state {
            State::Unassigned | State::Finished(..) => return Style::default(),
            State::Assigned(..) => Style::default(),
            State::Downloading(..) => Style::default().fg(Color::Blue),
            State::Booting(..) => Style::default().fg(Color::Yellow),
            State::Failed(..) => Style::default().fg(Color::Red),
            State::Reported(..) => Style::default().fg(Color::Green),
        }
        .add_modifier(Modifier::BOLD)
    }

    fn row(&self) -> Row<'static> {
        let style = self.style();
        let ip = self.state.ip().map(|ip| ip.to_string()).unwrap_or_default();
        let seen = self
            .seen
            .map(|time| DateTime::<Local>::from(time).format("%H:%M").to_string())
            .unwrap_or_default();

        Row::new(vec![
            Cell::from(self.state.emoji()),
            Cell::from(self.asset.name.clone()).style(style),
            Cell::from(ip).style(style),
            Cell::from(seen).style(style),
            Cell::from(format!("{:>10}", format_bytes(self.asset.size))).style(style),
        ])
    }
}

pub struct Update<'a>(&'a mut Status);

impl Deref for Update<'_> {
    type Target = Jobs;

    fn deref(&self) -> &Jobs {
        &self.0.jobs
    }
}

impl DerefMut for Update<'_> {
    fn deref_mut(&mut self) -> &mut Jobs {
        &mut self.0.jobs
    }
}

impl Drop for Update<'_> {
    fn drop(&mut self) {
        let _ = self.0.render();
    }
}

pub struct Status {
    jobs: Jobs,
    addr: SocketAddr,
    path: Arc<String>,
}

impl Status {
    pub fn new(assets: BTreeSet<Asset>, addr: SocketAddr, path: Arc<String>) -> Self {
        Self {
            jobs: Jobs::from(assets),
            addr,
            path,
        }
    }

    fn counts(&self) -> Paragraph<'static> {
        let mut unassigned = 0;
        let mut assigned = 0;
        let mut downloading = 0;
        let mut booting = 0;
        let mut reported = 0;
        let mut finished = 0;
        let mut failed = 0;

        for job in self.jobs.iter() {
            match job.state {
                State::Unassigned => unassigned += 1,
                State::Assigned(..) => assigned += 1,
                State::Downloading(..) => downloading += 1,
                State::Booting(..) => booting += 1,
                State::Reported(..) => reported += 1,
                State::Failed(..) => failed += 1,
                State::Finished(..) => finished += 1,
            }
        }

        let ip = IpAddr::V4([0, 0, 0, 0].into());
        let text = format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} 🚪 q",
            State::Unassigned.emoji(),
            unassigned,
            State::Assigned(ip).emoji(),
            assigned,
            State::Downloading(ip).emoji(),
            downloading,
            State::Booting(ip).emoji(),
            booting,
            State::Reported(ip).emoji(),
            reported,
            State::Finished(ip).emoji(),
            finished,
            State::Failed(ip).emoji(),
            failed,
        );

        Paragraph::new(text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
    }

    fn url(&self) -> Paragraph<'static> {
        Paragraph::new(format!("🌐 http://{}{}", self.addr, self.path))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
    }

    fn table(&self) -> Table<'static> {
        let jobs: BTreeSet<&Job> = self
            .jobs
            .iter()
            .filter(|job| !matches!(job.state, State::Finished(..)))
            .collect();

        let cells = vec![
            Cell::from(""),
            Cell::from("Job Name"),
            Cell::from("Assigned To"),
            Cell::from("Seen"),
            Cell::from(format!("{:>10}", "Size")),
        ];

        let style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);

        let header = Row::new(cells).style(style);

        let widths = [
            Constraint::Length(2),  // Status column
            Constraint::Min(20),    // Job Name column (flexible)
            Constraint::Min(15),    // IP Address column (flexible)
            Constraint::Length(5),  // Last Seen column
            Constraint::Length(10), // Size column
        ];

        let rows = jobs.into_iter().map(Job::row);
        Table::new(rows, widths).header(header).column_spacing(1)
    }

    pub fn render(&self) -> std::io::Result<()> {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(0), Constraint::Length(1)]);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Min(32)]);

        let url = self.url();
        let table = self.table();
        let counts = self.counts();

        super::TERMINAL.lock().unwrap().draw(move |f| {
            let chunks = vertical.split(f.area());
            f.render_widget(table, chunks[0]);

            let chunks = horizontal.split(chunks[1]);
            f.render_widget(url, chunks[0]);
            f.render_widget(counts, chunks[1]);
        })?;

        Ok(())
    }

    pub const fn update(&mut self) -> Update<'_> {
        Update(self)
    }
}
