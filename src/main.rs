use chrono::Local;
use clap::Parser;
use config::Args;
use crossterm::event;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use ratatui::{
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, enable_raw_mode},
    },
    prelude::*,
};
use std::{
    fs::File, io, thread, time::{Duration, Instant}
};

mod config;
mod http;
mod locales;
mod ok;
mod ui;
mod utils;

pub fn init_log() {
    let target = Box::new(File::create("log.txt").expect("Can't create file"));

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Info)
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log();

    let args = Args::parse();
    let c = config::Config::new(args.config.clone())?;

    locales::load(&c.lang)?;

    http::init().await?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let result = run_app().await;

    ratatui::restore();
    Ok(result?)
}

async fn close_position(p: ok::response::PositionResponse) {
    ok::close_position(&p.inst_id, &p.pos_side, &p.mgn_mode, &p.ccy)
        .await
        .expect("Failed to close position");
}

async fn run_app() -> anyhow::Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.show_cursor()?;

    let positions = ok::get_swap_positions().await?;
    let mut table = ui::PositonTable::new(positions);

    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();
    loop {
        if event::poll(Duration::from_millis(1))? {
            if let event::Event::Key(event) = event::read()? {
                if table.handle_input(event, close_position).await {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let positions = ok::get_swap_positions().await?;
            table.update(positions);
            last_tick = Instant::now();
        }

        terminal.draw(|f| table.draw(f))?;
        thread::sleep(Duration::from_millis(10));
    }

    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(())
}
