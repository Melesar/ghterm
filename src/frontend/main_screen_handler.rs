use crate::app::events::AppEvent;
use crate::backend::task::*;
use crate::backend::pr;

use super::screen::{Rect, ApplicationScreen, DrawableScreen, InteractableScreen, ScreenHandler};
use super::main_screen::MainScreen;

use std::io::{Write, Result};
use std::sync::mpsc;

pub struct MainScreenHandler {
    screen: MainScreen,
    event_sender: mpsc::Sender<AppEvent>,
    //task_handle: TaskHandle<Result<pr::Pr>>,
}

impl MainScreenHandler {
    pub fn new (number: u32, event_sender: mpsc::Sender<AppEvent>, task_manager: &mut TaskManager) -> Self {
        let screen = MainScreen::new(event_sender.clone());
        //let task_handle = task_manager.post(move || pr::fetch_pr(number));
        //MainScreenHandler { screen, event_sender, task_handle }*/
        MainScreenHandler{screen, event_sender}
    }
}

impl  ScreenHandler  for MainScreenHandler  {
    fn update(&mut self) {
    }
}

impl  DrawableScreen  for MainScreenHandler  {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
    }

}

impl  InteractableScreen for MainScreenHandler  {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}

impl ApplicationScreen for MainScreenHandler  {
}
