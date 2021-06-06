use json::JsonValue;

use crate::app::events::AppEvent;
use crate::backend::task::*;
use crate::backend::pr;
use crate::backend::gh::*;

use super::screen::{Rect, ApplicationScreen, DrawableScreen, InteractableScreen, ScreenHandler};
use super::main_screen::MainScreen;

use std::io::{Write, Result};
use std::sync::mpsc;

pub struct MainScreenHandler {
    screen: MainScreen,
    event_sender: mpsc::Sender<AppEvent>,
    conversation_task: TaskHandle<Result<JsonValue>>,
}

impl MainScreenHandler {
    pub fn new (number: u32, event_sender: mpsc::Sender<AppEvent>, task_manager: &mut TaskManager, client: &mut GhClient) -> Self {
        let screen = MainScreen::new(event_sender.clone());
        let mut request = client.pr_conversation(number).expect("Problem fetching pr conversation");
        let conversation_task = task_manager.post(move || request.execute());
        MainScreenHandler{screen, event_sender, conversation_task}
    }
}

impl  ScreenHandler  for MainScreenHandler  {
    fn update(&mut self) {
        if let Some(res) = self.conversation_task.poll() {
            match res { 
                Ok(json) => {
                    let conversation = pr::parse_conversation(json);
                    self.screen.set_conversation(conversation)
                },
                Err(error) => self.event_sender.send(AppEvent::Error(error.to_string())).unwrap()
            }
        }
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

impl ApplicationScreen for MainScreenHandler  {
}
