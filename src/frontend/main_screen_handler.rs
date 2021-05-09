use crate::app::events::AppEvent;
use crate::backend::pr;

use super::screen::{Rect, ApplicationScreen, DrawableScreen, InteractableScreen, ScreenHandler};
use super::main_screen::MainScreen;

use std::io::Write;
use std::sync::mpsc;
use std::thread;

pub struct MainScreenHandler {
    screen: MainScreen,
    event_sender: mpsc::Sender<AppEvent>,
    pr_receiver: mpsc::Receiver<std::io::Result<pr::Pr>>,
}

impl MainScreenHandler {
    pub fn new (number: u32, event_sender: mpsc::Sender<AppEvent>) -> Self {
        let screen = MainScreen::new(event_sender.clone());
        let (sender, pr_receiver) = mpsc::channel();
        thread::spawn(move || {
            let pr = pr::fetch_pr(number);
            sender.send(pr).unwrap();
        });
        MainScreenHandler{screen, event_sender, pr_receiver }
    }
}

impl  ScreenHandler  for MainScreenHandler  {
    fn update(&mut self) {
        if let Some(Ok(pr)) = self.pr_receiver.try_recv().ok() {
            self.screen.set_pr(pr);
            self.event_sender.send(AppEvent::ScreenRepaint).unwrap();
        };
    }
}

impl  DrawableScreen  for MainScreenHandler  {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        self.screen.draw(buffer, rect);
    }

}

impl  InteractableScreen for MainScreenHandler  {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}

impl  ApplicationScreen  for MainScreenHandler  {
}
