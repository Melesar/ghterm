use crate::app::events::AppEvent;

use super::screen::{Rect, ApplicationScreen, DrawableScreen, InteractableScreen, ScreenHandler};
use super::main_screen::MainScreen;

use std::io::Write;
use std::marker::PhantomData;
use std::sync::mpsc;

pub struct MainScreenHandler <W: Write> {
    screen: MainScreen<W>,
    event_sender: mpsc::Sender<AppEvent>,
    _marker: PhantomData<W>,
}

impl <W: Write> MainScreenHandler <W> {
    pub fn new (number: u32, event_sender: mpsc::Sender<AppEvent>) -> Self {
        let screen = MainScreen::new(event_sender.clone());
        MainScreenHandler{screen, event_sender, _marker: PhantomData}
    }
}

impl <W: Write> ScreenHandler <W> for MainScreenHandler <W> {
    fn update(&mut self) {
        
    }
}

impl <W: Write> DrawableScreen <W> for MainScreenHandler <W> {
    fn draw(&self, buffer: &mut W, rect: Rect) {
        self.screen.draw(buffer, rect);
    }

}

impl <W: Write> InteractableScreen for MainScreenHandler <W> {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}

impl <W: Write> ApplicationScreen <W> for MainScreenHandler <W> {
}
