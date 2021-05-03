use crate::app::events::AppEvent;
use crate::backend::pr::{self, PrHeader};
use crate::logs;

use super::screen::{DrawableScreen, InteractableScreen, Rect, ScreenHandler};
use super::repo_selection::RepoSelectionScreen;

use std::sync::mpsc;
use std::io::Write;
use std::thread;


pub struct RepoSelectionHandler<'a, W: Write> {
    buffer: &'a mut W,
    screen: RepoSelectionScreen,
    event_sender: mpsc::Sender<AppEvent>,
    repo_list_receiver: mpsc::Receiver<std::io::Result<Vec<PrHeader>>>,
    is_dirty: bool,
}

impl<'a, W: Write> RepoSelectionHandler<'a, W> { 
    pub fn new (buffer: &'a mut W, event_sender: mpsc::Sender<AppEvent>) -> Self {
        let (repo_list_sender, repo_list_receiver) = mpsc::channel::<std::io::Result<Vec<PrHeader>>>();
        thread::spawn(move || {
            let prs = pr::list_prs();
            repo_list_sender.send(prs).unwrap();
        });

        let screen = RepoSelectionScreen::new(event_sender.clone());
        RepoSelectionHandler {buffer, screen, event_sender, repo_list_receiver, is_dirty: true}
    }
}

impl<'a, W: Write> ScreenHandler<'a, W> for RepoSelectionHandler<'a, W> {
    fn update(&mut self, application_rect: Rect) {
        if self.is_dirty {
            self.screen.draw(self.buffer, application_rect);
            self.is_dirty = false;
        }

        match self.repo_list_receiver.try_recv().ok() {
            Some(ok) => match ok {
                Ok(prs) => {
                    self.screen.set_pr_list(prs);
                    self.is_dirty = true;
                },
                Err(error) => {
                    self.event_sender.send(AppEvent::Error(error.to_string())).unwrap();
                }
            }
            None => (),
        }
    }
}

impl<'a, W: Write> InteractableScreen for RepoSelectionHandler<'a, W> {
    fn validate_input(&self, b: u8) -> bool {
        self.screen.validate_input(b)
    }

    fn process_input(&mut self, b: u8) -> bool {
        self.screen.process_input(b)
    }
}
