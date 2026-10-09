use crate::{
    directory::{Response, Status},
    native,
    navigation::{Intent, Navigation},
    sorting::Key,
};
use eframe::egui;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TabId(pub u64);

pub struct Tab {
    pub id: TabId,
    pub navigation: Navigation,
    pub requested_sort: Option<Key>,
    pub location_text: String,
    pub location_editor_focused: bool,
    pub location_error: Option<String>,
    pub filter_text: String,
    pub row_scroll_offset: f32,
    pub row_viewport_height: f32,
    pub scroll_to_active: bool,
    pub restore_scroll: bool,
    synchronize_location: bool,
}

impl Tab {
    pub fn new(id: TabId, location: PathBuf) -> Self {
        Self {
            id,
            location_text: native::display(location.as_os_str()),
            navigation: Navigation::new(location),
            requested_sort: None,
            location_editor_focused: false,
            location_error: None,
            filter_text: String::new(),
            row_scroll_offset: 0.0,
            row_viewport_height: 0.0,
            scroll_to_active: false,
            restore_scroll: true,
            synchronize_location: true,
        }
    }

    pub fn widget_id(&self, name: &'static str) -> egui::Id {
        egui::Id::new((self.id, name))
    }

    pub fn begin(&mut self, location: PathBuf, generation: u64, intent: Intent) {
        self.location_error = None;
        self.synchronize_location = true;
        self.requested_sort = None;
        self.restore_scroll = true;
        self.scroll_to_active = false;
        self.navigation.begin(location, generation, intent);
    }

    pub fn resume(&mut self, generation: u64) {
        let error = self.location_error.take();
        self.begin(
            self.navigation.current.location.clone(),
            generation,
            Intent::Refresh,
        );
        self.location_error = error;
        // A retained path-editor draft is not replaced by an activation refresh.
        self.synchronize_location = false;
    }

    pub fn deactivate(&mut self) {
        // Only the owning pane's active tab retains a worker target. Interrupted destinations
        // never commit later; returning refreshes the committed location.
        self.navigation.deactivate();
        self.requested_sort = None;
        self.location_editor_focused = false;
        self.scroll_to_active = false;
        self.restore_scroll = true;
    }

    pub fn apply(&mut self, response: Response) -> bool {
        let previous = self.navigation.current.location.clone();
        let committed = self.navigation.apply(response);
        if committed {
            if self.navigation.current.location != previous {
                self.row_scroll_offset = 0.0;
            }
            if self.synchronize_location && !self.location_editor_focused {
                self.location_text = native::display(self.navigation.current.location.as_os_str());
            }
        }
        committed
    }

    pub fn scroll_restore_ready(&self) -> bool {
        self.navigation.pending_location().is_none()
            && matches!(self.navigation.current.status, Status::Ready)
            && self.navigation.projection_fresh()
    }
}
