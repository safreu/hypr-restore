use crate::tui::{App, Tab};
use crossterm::event::{KeyCode, KeyEvent};
pub(crate) fn pipeline_handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.exit(),
        KeyCode::Tab => app.tab_state.next(),
        KeyCode::Char('r') => app.show_popup = !app.show_popup,
        _ => pipeline_to_context(app, key_event),
    }
}

fn pipeline_to_context(app: &mut App, key_event: KeyEvent) {
    if app.show_popup {
        key_event_pop_up(app, key_event);
        return;
    }
    match app.tab_state.active_tab() {
        Tab::FileContent => key_event_file_content(app, key_event),
        Tab::CustomWorkspace => {}
        _ => {}
    }
}

fn key_event_pop_up(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up => app.tab_state.move_up_entries(),
        KeyCode::Down => app.tab_state.move_down_entries(),
        KeyCode::Enter => app.tab_state.run_command(),
        _ => {}
    }
}

fn key_event_file_content(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Up => app.content_provider.move_up_contents(),
        KeyCode::Down => app.content_provider.move_down_contents(),
        _ => {}
    }
}
