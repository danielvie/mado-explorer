pub use mado_core::finder::*;

// Input focus belongs to the GUI, not the shared finder state.
pub struct Modal {
    state: mado_core::finder::Modal,
    pub focus_input: bool,
}

impl Modal {
    pub fn new(target: Target) -> Self {
        Self {
            state: mado_core::finder::Modal::new(target),
            focus_input: true,
        }
    }

    pub fn accept(&mut self, response: Response) -> Option<(std::path::PathBuf, Action)> {
        if response.revision == self.state.revision
            && matches!(response.output, Output::Expanded(..))
        {
            self.focus_input = true;
        }
        self.state.accept(response)
    }
}

impl std::ops::Deref for Modal {
    type Target = mado_core::finder::Modal;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl std::ops::DerefMut for Modal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
