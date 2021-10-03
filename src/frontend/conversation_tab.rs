mod conversation_tree;
mod conversation_draw;

use std::io::Write;
use std::sync::mpsc;

use crate::backend::pr::{ ConversationItem, PrComment, PrConversation, PrReview };

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;
use conversation_tree::ConversationTree;

pub struct ConversationTab {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation_tree: Option<ConversationTree>,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab { screen_event_sender, conversation_tree: None }
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation_tree = Some(ConversationTree::new(conversation));
    }
}

impl DrawableScreen for ConversationTab {

    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        let mut left_part = rect.screen();
        let right_part = left_part.split_vertically();
        let mut writer = left_part.get_content_rect().screen().get_writer();

        write!(buffer, "{}", termion::clear::All).unwrap();

        if let Some(conversation_tree) = self.conversation_tree.as_ref() {
            conversation_tree.draw(buffer, &mut writer);
        }

        left_part.draw_border(buffer);
        right_part.draw_border(buffer);
        buffer.flush().unwrap();
    }
}

impl InteractableScreen for ConversationTab {
    fn validate_input(&self, input: u8) -> bool {
        self.conversation_tree.is_some() &&
            (input == b'j' || input == b'k' || input == b'h' || input == b'l' ||
             input == b' ')
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
        
        let tree = self.conversation_tree.as_mut().unwrap();
        if input == b' ' {
            tree.toggle_expansion();
        }
        else if vertical_offset != 0 {
            tree.move_selection(vertical_offset > 0);
        } 
    }
}

impl ApplicationScreen for ConversationTab {
}
