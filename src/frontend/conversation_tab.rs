use std::io::Write;
use std::sync::mpsc;
use std::iter::FromIterator;

use crate::backend::pr::{PrConversation, ConversationItem, PrComment};

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

pub struct ConversationTab  {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation_items: Vec<PrComment>,
    selected_conversation: usize,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab{screen_event_sender, conversation_items: vec![], selected_conversation: 0}
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation_items = conversation.items
            .into_iter()
            .map(|e| match e {
                ConversationItem::Review(r) => r.review_comment,
                ConversationItem::Comment(c) => c,
            })
            .collect();
    }
}

impl DrawableScreen for ConversationTab {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        let mut conversation_screen = Screen::new(rect);
        let details_screen = conversation_screen.split_vertically();
        let mut writer = conversation_screen.get_content_rect().screen().get_writer();
        for (index, comment) in self.conversation_items.iter().enumerate() {
            writer.set_selection(index == self.selected_conversation);
            writer.write_line(buffer, &comment.author_name);
            writer.write_line(buffer, &comment.body);
            writer.set_selection(false);
            writer.separator(buffer);
        }

        conversation_screen.draw_border(buffer);
        details_screen.draw_border(buffer);

        buffer.flush().unwrap();
    }
}

impl  InteractableScreen for ConversationTab {
    fn validate_input(&self, input: u8) -> bool {
        self.conversation_items.len() > 0 && (input == b'j' || input == b'k')
    }

    fn process_input(&mut self, input: u8) {
        let offset = match input {
            b'j' => 1,
            b'k' => -1,
            _ => 0
        };
        
        let mut current_index = self.selected_conversation as i32;
        current_index = (current_index + offset).max(0).min((self.conversation_items.len() - 1) as i32);
        self.selected_conversation = current_index as usize;
    }
}

impl  ApplicationScreen for ConversationTab {
}
