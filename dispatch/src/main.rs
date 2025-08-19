#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]
#![deny(unreachable_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

mod avahi;
mod github;
mod http;
mod jobs;
mod tui;

use std::sync::Arc;

use crate::avahi::AvahiService;
use crate::github::GitHub;
use crate::http::Server;
use crate::tui::{Status, Throbbing};

use anyhow::Result;
use clap::Parser;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind};
use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[derive(Parser)]
#[command(name = std::env!("CARGO_PKG_NAME"))]
#[command(about = std::env!("CARGO_PKG_DESCRIPTION"))]
struct Args {
    #[command(flatten)]
    github: GitHub,

    /// Address and port to bind to
    #[arg(short = 'b', long, default_value = "0.0.0.0:8080")]
    bind: String,

    /// Path to offer services on
    #[arg(short = 'p', long, default_value = concat!("/", std::env!("CARGO_PKG_NAME")))]
    path: String,
}

#[tokio::main]
#[allow(clippy::too_many_lines)]
async fn main() -> Result<()> {
    // Parse arguments
    let mut args = Args::parse();

    // Ensure we're authenticated with GitHub
    args.github.login()?;

    // Bind to our server port
    let listener = TcpListener::bind(&args.bind).await?;
    let addr = listener.local_addr()?;
    let path = Arc::new(args.path);

    // Load the github assets
    let assets = args
        .github
        .assets()
        .throbbing("Loading GitHub assets...")
        .await?;

    // Show the main UI
    let status = Arc::new(Mutex::new(Status::new(assets, addr, path.clone())));
    status.lock().await.render()?;

    // Create the HTTP server
    let github = Arc::new(args.github);
    let server = Server::new(listener, status.clone(), github, path.clone())?;

    // Create TXT records
    let name = std::env!("CARGO_PKG_NAME");
    let txt = [
        ("description", std::env!("CARGO_PKG_DESCRIPTION")),
        ("version", std::env!("CARGO_PKG_VERSION")),
        ("path", &path),
    ];

    // Start the Avahi service discovery.
    let avahi = AvahiService::new().await?;
    avahi.register(name, addr.port(), &txt).await?;

    // Create event stream for terminal events
    let mut events = EventStream::new();

    // Run the server and wait for quit or terminal events in parallel
    tokio::select! {
        _ = server.serve() => {}
        _ = terminal_events(&mut events, status.clone()) => {}
    }

    ratatui::restore();
    Ok(())
}

async fn terminal_events(events: &mut EventStream, status: Arc<Mutex<Status>>) -> Result<()> {
    loop {
        if let Some(event) = events.next().await {
            match event? {
                // Quit on 'q' key press
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                }) => break,

                // Re-render the status on terminal resize
                Event::Resize(_, _) => status.lock().await.render()?,

                // Ignore other events
                _ => {}
            }
        }
    }

    Ok(())
}
