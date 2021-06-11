use std::io::Write;
use std::sync::mpsc;

use crate::backend::pr::PrConversation;

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

pub struct ConversationTab  {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation: Option<PrConversation>,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab{screen_event_sender, conversation: None}
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation = Some(conversation);
    }
}

impl DrawableScreen for ConversationTab {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
    }
}

impl  InteractableScreen for ConversationTab {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}

impl  ApplicationScreen for ConversationTab {
}
