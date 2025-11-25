use crate::tui::popup::{FileContentPopUp, ListenerServicePopUp, Popup, SnapshotServicePopUp};
use ratatui::Frame;
use ratatui::prelude::Rect;

#[derive(Default, Debug, PartialEq, Eq)]
pub enum Tab {
    #[default]
    FileContent,
    ListenerService,
    RestoreService,
    CustomWorkspace,
}

#[derive(Default, Debug, PartialEq)]
pub struct TabState {
    active_tab: Tab,
    pub file_content_popup: Popup<FileContentPopUp>,
    pub listener_popup: Popup<ListenerServicePopUp>,
    pub restore_popup: Popup<SnapshotServicePopUp>,
    pub custom_workspace_popup: Popup<FileContentPopUp>,
}

impl TabState {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::FileContent,
            file_content_popup: Popup::new(
                vec!["snapshot".to_string(), "restore".to_string()],
                FileContentPopUp,
            ),
            listener_popup: Popup::new(
                vec![
                    "start".to_string(),
                    "restart".to_string(),
                    "stop".to_string(),
                    "enable".to_string(),
                    "disable".to_string(),
                ],
                ListenerServicePopUp,
            ),
            restore_popup: Popup::new(
                vec![
                    "start".to_string(),
                    "restart".to_string(),
                    "disable".to_string(),
                    "stop".to_string(),
                ],
                SnapshotServicePopUp,
            ),
            custom_workspace_popup: Popup::new(
                vec![
                    "create".to_string(),
                    "remove".to_string(),
                    "change order".to_string(),
                    "run".to_string(),
                ],
                FileContentPopUp,
            ),
        }
    }

    pub fn active_tab(&self) -> &Tab {
        &self.active_tab
    }

    pub fn next(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::FileContent => Tab::ListenerService,
            Tab::ListenerService => Tab::RestoreService,
            Tab::RestoreService => Tab::CustomWorkspace,
            Tab::CustomWorkspace => Tab::FileContent,
        }
    }

    pub fn move_up_entries(&mut self) {
        match &self.active_tab {
            Tab::FileContent => self.file_content_popup.move_up_entries(),
            Tab::ListenerService => self.listener_popup.move_up_entries(),
            Tab::RestoreService => self.restore_popup.move_up_entries(),
            Tab::CustomWorkspace => self.custom_workspace_popup.move_up_entries(),
        }
    }

    pub fn run_command(&mut self) {
        match &self.active_tab {
            Tab::FileContent => self.file_content_popup.run_command(),
            Tab::ListenerService => self.listener_popup.run_command(),
            Tab::RestoreService => self.restore_popup.run_command(),
            Tab::CustomWorkspace => self.custom_workspace_popup.run_command(),
        }
    }
    pub fn move_down_entries(&mut self) {
        match &self.active_tab {
            Tab::FileContent => self.file_content_popup.move_down_entries(),
            Tab::ListenerService => self.listener_popup.move_down_entries(),
            Tab::RestoreService => self.restore_popup.move_down_entries(),
            Tab::CustomWorkspace => self.custom_workspace_popup.move_down_entries(),
        }
    }
    pub fn draw_popup(&self, area: Rect, frame: &mut Frame) {
        match &self.active_tab {
            Tab::FileContent => self.file_content_popup.draw_popup(area, frame),
            Tab::ListenerService => self.listener_popup.draw_popup(area, frame),
            Tab::RestoreService => self.restore_popup.draw_popup(area, frame),
            Tab::CustomWorkspace => self.custom_workspace_popup.draw_popup(area, frame),
        }
    }
}
