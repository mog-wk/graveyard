use ratatui::{
    backend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::Backend,
    widgets::Clear,
    Terminal,
};

use crate::app::{App, AppState};
use crate::error::Result;

const TICK_RATE: std::time::Duration = std::time::Duration::from_millis(200);

pub fn run() -> Result<()> {
    let span = tracing::span!(tracing::Level::INFO, "run");
    let _guard = span.enter();

    enable_raw_mode()?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = backend::CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let app_result = run_app(&mut terminal, App::new());

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        tracing::error!("{:?}", err);
    }

    Ok(())
}

fn run_app<B>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    B: Backend,
{
    let last_tick = std::time::Instant::now();
    loop {
        if app.state == AppState::Exit {
            break;
        }

        terminal.draw(|frame| {
            frame.render_widget(Clear, frame.area());
            frame.render_widget(&mut app, frame.area());
        })?;

        let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.state = AppState::Exit,
                        KeyCode::Char('j') => (),
                        _ => tracing::info!("pressed: {:?}", key),
                    }
                }
            }
        }
    }

    Ok(())
}
