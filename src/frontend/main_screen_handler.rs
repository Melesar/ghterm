use crate::backend::diff::ChangeList;
use json::JsonValue;
use crate::app::events::AppEvent;
use crate::backend::task::*;
use crate::backend::pr;
use crate::backend::gh::*;
use crate::error::Error;

use super::screen::{Rect, ApplicationScreen, DrawableScreen, InteractableScreen, ScreenHandler};
use super::main_screen::MainScreen;

use std::io::Write;
use std::sync::mpsc;

pub enum MainScreenEvent {

}

pub struct MainScreenHandler<'a> {
    screen: MainScreen,
    app_events_sender: mpsc::Sender<AppEvent>,
    conversation_task: TaskHandle<Result<JsonValue, Error>>,
    diff_task: TaskHandle<Result<String, Error>>,
    task_manager: TaskManager,
    client: &'a GhClient,
    screen_events_receiver: mpsc::Receiver<MainScreenEvent>,
}

impl<'a> MainScreenHandler<'a> {
    pub fn new (number: u32, app_events_sender: mpsc::Sender<AppEvent>, client: &'a GhClient) -> Self {
        let mut conversation_request = client.pr_conversation(number).expect("Problem fetching pr conversation");
        let mut diff_request = client.pr_diff(number);
        let mut task_manager = TaskManager::new(app_events_sender.clone());
        let conversation_task = task_manager.post(move || conversation_request.execute());
        let diff_task = task_manager.post(move || diff_request.execute());
        
        let (events_tx, screen_events_receiver) = mpsc::channel();
        let screen = MainScreen::new(app_events_sender.clone(), events_tx.clone());

        MainScreenHandler{
            screen,
            app_events_sender,
            conversation_task,
            diff_task,
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

        if let Some(res) = self.conversation_task.poll() {
            match res { 
                Ok(json) => {
                    let conversation = pr::parse_conversation(json);
                    self.screen.set_conversation(conversation);
                },
                Err(error) => self.app_events_sender.send(AppEvent::Error(error.to_string())).unwrap()
            }
        }

        if let Some(diff) = self.diff_task.poll() {
            match diff {
                Ok(diff) => {
                    let changelist = ChangeList::new(diff);
                    self.screen.set_changelist(changelist);
                },
                Err(error) => self.app_events_sender.send(AppEvent::Error(error.to_string())).unwrap()
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
    fn validate_input(&self, input: termion::event::Key) -> bool {
        self.screen.validate_input(input)
    }

    fn process_input(&mut self, input: termion::event::Key) {
        self.screen.process_input(input);
    }
}

impl<'a> ApplicationScreen for MainScreenHandler<'a> {
}
