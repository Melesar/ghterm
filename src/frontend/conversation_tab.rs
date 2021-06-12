use std::io::Write;
use std::sync::mpsc;
use std::iter::FromIterator;

use crate::backend::pr::{PrConversation, ConversationItem};

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

pub struct ConversationTab  {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation: Option<PrConversation>,
    selected_conversation: usize,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab{screen_event_sender, conversation: None, selected_conversation: 0}
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation = Some(conversation);
    }
}

impl DrawableScreen for ConversationTab {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        if let Some(conv) = &self.conversation {
            let comments : Vec<_> = conv.items
                .iter()
                .map(|e| match e {
                    ConversationItem::Review(r) => &r.review_comment,
                    ConversationItem::Comment(c) => &c,
                })
                .collect();

            let screen = Screen::new(rect);
            let mut writer = screen.get_writer();
            for (index, comment) in comments.into_iter().enumerate() {
                writer.set_selection(index == self.selected_conversation);
                writer.write_line(buffer, &comment.author_name);
                writer.write_line(buffer, &comment.body);
                writer.set_selection(false);
                writer.separator(buffer);
            }
            buffer.flush().unwrap();
        }
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
