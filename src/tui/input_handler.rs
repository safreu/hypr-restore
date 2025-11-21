use crate::tui::{App, Focused};
use crossterm::event::{KeyCode, KeyEvent};
pub(crate) fn pipeline_handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Tab => switch_focused(app),
        _ => match app.focused {
            Focused::TopLeft => key_event_top_left(app, key_event),
            Focused::TopRight => key_event_top_right(app, key_event),
            Focused::Bottom => key_event_bottom(app, key_event),
            Focused::PopUp => key_event_pop_up(app, key_event),
        },
    };
}

fn switch_focused(app: &mut App) {
    if app.focused == Focused::TopLeft {
        app.focused = Focused::TopRight
    } else if app.focused == Focused::TopRight {
        app.focused = Focused::Bottom
    } else if app.focused == Focused::Bottom {
        app.focused = Focused::TopLeft
    }
}

fn key_event_pop_up(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up => app.popup.move_up_commands(),
        KeyCode::Down => app.popup.move_down_commands(),
        KeyCode::Char('r') => {
            app.show_popup = false;
            app.focused = Focused::TopLeft;
        }
        KeyCode::Enter => app.popup.run_command(),
        _ => {}
    }
}

fn key_event_bottom(app: &App, key_event: KeyEvent) {
    todo!()
}

fn key_event_top_right(app: &App, key_event: KeyEvent) {
    todo!()
}

fn key_event_top_left(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up => app.content_provider.move_up_contents(),
        KeyCode::Down => app.content_provider.move_down_contents(),
        KeyCode::Char('r') => {
            app.show_popup = true;
            app.focused = Focused::PopUp;
        }
        _ => {}
    }
}

