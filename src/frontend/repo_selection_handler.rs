use crate::app::events::AppEvent;
use crate::backend::task::*;
use crate::backend::pr::{self, PrHeader};
use crate::backend::gh::GhClient;
use crate::error::Error;
use json::JsonValue;
use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;

use super::screen::*;
use super::repo_selection::RepoSelectionScreen;

use std::sync::mpsc;

pub struct RepoSelectionHandler {
    screen: RepoSelectionScreen,
    event_sender: mpsc::Sender<AppEvent>,
    task_handle: TaskHandle<Result<JsonValue, Error>>,
}

impl RepoSelectionHandler { 
    pub fn new (event_sender: mpsc::Sender<AppEvent>, task_manager: &mut TaskManager, client: &mut GhClient) -> Self {
        let mut request = client.pr_list().expect("Wasn't able to fetch prs");
        let task_handle = task_manager.post(move || request.execute());
        let screen = RepoSelectionScreen::new(event_sender.clone());
        RepoSelectionHandler {screen, event_sender, task_handle}
    }
}

impl<B: Backend> ScreenHandler<B> for RepoSelectionHandler {
    fn update(&mut self) {
        match self.task_handle.poll() {
            Some(ok) => match ok {
                Ok(json) => {
                    let prs = pr::list_prs(json);
                    self.screen.set_pr_list(prs)
                },
                Err(error) => self.event_sender.send(AppEvent::Error(error.to_string())).unwrap(),
            }
            None => (),
        }
    }
}

impl InteractableScreen for RepoSelectionHandler {
    fn validate_input(&self, b: Key) -> bool {
        self.screen.validate_input(b)
    }

    fn process_input(&mut self, b: Key) {
        self.screen.process_input(b);
    }
}

impl<B: Backend> DrawableScreen<B>  for RepoSelectionHandler  {
    fn draw (&self, frame: &mut Frame<B>) {
        self.screen.draw(frame);
    }
}

impl<B: Backend> ApplicationScreen<B>  for RepoSelectionHandler  {
}
