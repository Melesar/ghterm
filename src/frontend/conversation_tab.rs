mod conversation_data;

use std::io::Write;
use std::sync::mpsc;

use conversation_data::ConversationData;
use crate::backend::pr::{ ConversationItem, PrComment, PrConversation, PrReview };

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

struct DummyConversation;

pub struct ConversationTab {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation: Box<dyn ConversationData>,
    selected_conversation: usize,
    selected_thread: Option<usize>,
    selected_comment: Option<usize>,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        let conversation = Box::new(DummyConversation{});
        ConversationTab { screen_event_sender, conversation, selected_conversation: 0, selected_thread: None, selected_comment: None }
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation = Box::new(conversation);
    }
}

impl DrawableScreen for ConversationTab {

    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        let conversation_screen = rect.screen();
        let mut writer = conversation_screen.get_content_rect().screen().get_writer();

        write!(buffer, "{}", termion::clear::All);
        self.conversation.draw(&mut writer, buffer, self.selected_conversation, self.selected_thread, self.selected_comment);

        conversation_screen.draw_border(buffer);
        buffer.flush().unwrap();
    }
}

impl InteractableScreen for ConversationTab {
    fn validate_input(&self, input: u8) -> bool {
        input == b'j' || input == b'k' || input == b'h' || input == b'l'
    }

    fn process_input(&mut self, input: u8) {
        let vertical_offset = match input {
            b'j' => 1,
            b'k' => -1,
            _ => 0
        };
        let horizontal_offset = match input {
            b'h' => -1, 
            b'l' => 1,
            _ => 0,
        };

        self.conversation.try_move(horizontal_offset, vertical_offset, &mut self.selected_conversation, &mut self.selected_thread, &mut self.selected_comment);
    }
}

impl ApplicationScreen for ConversationTab {
}

impl ConversationData for DummyConversation {
    fn draw(&self, _: &mut ScreenWriter, _: &mut dyn Write, _: usize, _: Option<usize>, _: Option<usize>) { }
    fn try_move(&self, _: i32, _: i32, _: &mut usize, _: &mut Option<usize>, _: &mut Option<usize>) { }
}
