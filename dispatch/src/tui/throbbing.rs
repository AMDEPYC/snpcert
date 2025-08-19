use std::time::Duration;

use anyhow::Result;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::widgets::Paragraph;
use tokio::time::{interval, Instant};

use super::TERMINAL;

const SPINNER: &[char] = &['◐', '◓', '◑', '◒'];

/// Extension trait to add spinner functionality to any future
pub trait Throbbing<T> {
    /// Show a spinner while waiting for this future to complete
    async fn throbbing(self, label: &str) -> Result<T>;
}

impl<F, T> Throbbing<T> for F
where
    F: std::future::Future<Output = Result<T>>,
{
    async fn throbbing(self, label: &str) -> Result<T> {
        let start = Instant::now();
        let mut tick = 0;
        let mut interval = interval(Duration::from_millis(100));

        let future = self;
        tokio::pin!(future);

        loop {
            tokio::select! {
                result = &mut future => return result,

                _ = interval.tick() => {
                    let elapsed = start.elapsed().as_secs_f32();
                    let spinner = SPINNER[tick % SPINNER.len()];
                    let text = format!("{spinner} {label} ({elapsed:.1}s)");

                    TERMINAL.lock().unwrap().draw(|f| {
                        let area = Layout::default()
                            .constraints([Constraint::Fill(1), Constraint::Length(1), Constraint::Fill(1)])
                            .split(f.area())[1];

                        let widget = Paragraph::new(text).alignment(Alignment::Center);

                        f.render_widget(widget, area);
                    }).ok();

                    tick += 1;
                }
            }
        }
    }
}
