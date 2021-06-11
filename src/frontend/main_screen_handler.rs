use json::JsonValue;
use crate::app::events::AppEvent;
use crate::backend::task::*;
use crate::backend::pr;
use crate::backend::gh::*;

use super::screen::{Rect, ApplicationScreen, DrawableScreen, InteractableScreen, ScreenHandler};
use super::main_screen::MainScreen;

use std::io::Write;
use std::sync::mpsc;

pub enum MainScreenEvent {

}

pub struct MainScreenHandler<'a> {
    screen: MainScreen,
    app_events_sender: mpsc::Sender<AppEvent>,
    conversation_task: TaskHandle<Result<JsonValue, GhError>>,
    task_manager: TaskManager,
    client: &'a GhClient,
    screen_events_receiver: mpsc::Receiver<MainScreenEvent>,
}

impl<'a> MainScreenHandler<'a> {
    pub fn new (number: u32, app_events_sender: mpsc::Sender<AppEvent>, client: &'a GhClient) -> Self {
        let mut request = client.pr_conversation(number).expect("Problem fetching pr conversation");
        let mut task_manager = TaskManager::new();
        let conversation_task = task_manager.post(move || request.execute());
        
        let (events_tx, screen_events_receiver) = mpsc::channel();
        let screen = MainScreen::new(app_events_sender.clone(), events_tx.clone());

        MainScreenHandler{
            screen,
            app_events_sender,
            conversation_task,
            task_manager,
            client,
            screen_events_receiver,
        }
    }
}

impl<'a> ScreenHandler for MainScreenHandler<'a> {
    fn update(&mut self) {
        if let Some(evt) = self.screen_events_receiver.try_recv().ok() {

        }

        if self.task_manager.update() == 0 {
            return;
        }

        if let Some(res) = self.conversation_task.poll() {
            match res { 
                Ok(json) => {
                    let conversation = pr::parse_conversation(json);
                    self.screen.set_conversation(conversation);
                },
                Err(error) => self.app_events_sender.send(AppEvent::Error(error.message)).unwrap()
            }
        }
    }
}

impl<'a> DrawableScreen for MainScreenHandler<'a> {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        self.screen.draw(buffer, rect);
    }

}

impl<'a> InteractableScreen for MainScreenHandler<'a> {
    fn validate_input(&self, input: u8) -> bool {
        self.screen.validate_input(input)
    }

    fn process_input(&mut self, input: u8) {
        self.screen.process_input(input);
    }
}

impl<'a> ApplicationScreen for MainScreenHandler<'a> {
}
