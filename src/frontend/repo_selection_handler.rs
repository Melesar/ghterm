use crate::app::events::AppEvent;
use crate::backend::pr::{self, PrHeader};

use super::screen::{ApplicationScreen, DrawableScreen, InteractableScreen, Rect, ScreenHandler};
use super::repo_selection::RepoSelectionScreen;

use std::sync::mpsc;
use std::io::Write;
use std::thread;
use std::marker::PhantomData;

pub struct  RepoSelectionHandler <W: Write> {
    screen: RepoSelectionScreen<W>,
    event_sender: mpsc::Sender<AppEvent>,
    repo_list_receiver: mpsc::Receiver<std::io::Result<Vec<PrHeader>>>,
    _marker: PhantomData<W>,
}

impl<W: Write> RepoSelectionHandler <W> { 
    pub fn new (event_sender: mpsc::Sender<AppEvent>) -> Self {
        let (repo_list_sender, repo_list_receiver) = mpsc::channel::<std::io::Result<Vec<PrHeader>>>();
        thread::spawn(move || {
            let prs = pr::list_prs();
            repo_list_sender.send(prs).unwrap();
        });

        let screen = RepoSelectionScreen::new(event_sender.clone());
        RepoSelectionHandler {screen, event_sender, repo_list_receiver, _marker: PhantomData}
    }
}

impl <W: Write> ScreenHandler<W> for RepoSelectionHandler <W> {
    fn update(&mut self) {
        match self.repo_list_receiver.try_recv().ok() {
            Some(ok) => match ok {
                Ok(prs) => {
                    self.screen.set_pr_list(prs);
                },
                Err(error) => {
                    self.event_sender.send(AppEvent::Error(error.to_string())).unwrap();
                }
            }
            None => (),
        }
    }
}

impl<W: Write> InteractableScreen for RepoSelectionHandler<W> {
    fn validate_input(&self, b: u8) -> bool {
        self.screen.validate_input(b)
    }

    fn process_input(&mut self, b: u8) {
        self.screen.process_input(b);
    }
}

impl <W: Write> DrawableScreen <W> for RepoSelectionHandler <W> {
    fn draw (&self, buffer: &mut W, rect: Rect) {
        self.screen.draw(buffer, rect);
    }
}

impl <W: Write> ApplicationScreen <W> for RepoSelectionHandler <W> {
}
