use std::io::Write;
use std::sync::mpsc;

use crate::backend::pr::{ConversationItem, PrComment, PrConversation, PrReview};

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

    fn write_review(writer: &mut ScreenWriter, buffer: &mut dyn Write, review: &PrReview) {
        writer.write_line(buffer, &format!("[R] {}\t{}", review.review_comment.author_name, review.verdict));
        if review.review_comment.body.len() > 0 {
            writer.write_line(buffer, "");
            writer.set_indent(1);
            writer.write_line(buffer, &review.review_comment.body);
            writer.set_indent(0);
            writer.write_line(buffer, "");
        }
        let num_threads = review.threads.len();
        let letter = if num_threads > 1 { "s" } else { "" };
        writer.write_line(buffer, &format!("{} thread{}", num_threads, letter));
    }
}

impl DrawableScreen for ConversationTab {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        if let Some(conversation) = &self.conversation {
            let conversation_screen = rect.screen();
            let mut writer = conversation_screen.get_content_rect().screen().get_writer();
            for (index, item) in conversation.items.iter().enumerate() {
                writer.set_selection(index == self.selected_conversation);
                match item {
                    ConversationItem::Review(r) => ConversationTab::write_review(&mut writer, buffer, &r),
                    ConversationItem::Comment(c) => {
                        writer.write_line(buffer, &format!("[C] {}", c.author_name));
                        writer.write_line(buffer, "");
                        writer.write_line(buffer, &c.body);
                    },
                }
                writer.set_selection(false);
                writer.separator(buffer);
            }

            conversation_screen.draw_border(buffer);
            buffer.flush().unwrap();
        }
    }
}

impl  InteractableScreen for ConversationTab {
    fn validate_input(&self, input: u8) -> bool {
        self.conversation.is_some() && (input == b'j' || input == b'k')
    }

    fn process_input(&mut self, input: u8) {
        let offset = match input {
            b'j' => 1,
            b'k' => -1,
            _ => 0
        };
        
        let conversation = self.conversation.iter().next().unwrap();
        let mut current_index = self.selected_conversation as i32;
        current_index = (current_index + offset).max(0).min((conversation.items.len() - 1) as i32);
        self.selected_conversation = current_index as usize;
    }
}

impl  ApplicationScreen for ConversationTab {
}
