use std::io::Write;
use std::sync::mpsc;

use crate::backend::pr::{ConversationItem, PrComment, PrConversation, PrReview};

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

enum View {
    Conversation,
    Thread,
    Comment
}

pub struct ConversationTab  {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation: Option<PrConversation>,
    selected_conversation: usize,
    selected_thread: i32,
    current_view: View,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab { screen_event_sender, conversation: None, selected_conversation: 0, selected_thread: -1, current_view: View::Conversation }
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

    fn write_comment(writer: &mut ScreenWriter, buffer: &mut dyn Write, comment: &PrComment) {
        writer.write_line(buffer, &format!("[C] {}", comment.author_name));
        writer.write_line(buffer, "");
        writer.write_line(buffer, &comment.body);
    }

    fn write_items<T, F>(writer: &mut ScreenWriter, buffer: &mut dyn Write, selected_item: i32, iter: &mut dyn Iterator<Item = T>, func: F) 
        where F : Fn(T, &mut ScreenWriter, &mut dyn Write) {
        for (index, item) in iter.enumerate() {
            writer.set_selection(index as i32 == selected_item);
            func(item, writer, buffer);
            writer.set_selection(false);
            writer.separator(buffer);
        }
    }
}

impl DrawableScreen for ConversationTab {

    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        if let None = &self.conversation {
            return;
        }
        
        let conversation = self.conversation.as_ref().unwrap();
        if let None = conversation.items.get(self.selected_conversation) {
            return;
        }

        let selected_conversation = conversation.items.get(self.selected_conversation).unwrap();
        let conversation_screen = rect.screen();
        let mut writer = conversation_screen.get_content_rect().screen().get_writer();
        match self.current_view {
            View::Conversation => {
                ConversationTab::write_items(&mut writer, buffer, self.selected_conversation as i32, &mut conversation.items.iter(), |item, writer, buffer| {
                    match item {
                        ConversationItem::Review(r) => ConversationTab::write_review(writer, buffer, &r),
                        ConversationItem::Comment(c) => ConversationTab::write_comment(writer, buffer, c),
                    }
                });
            },
            View::Thread => {
                if let ConversationItem::Review(review) = selected_conversation {
                    ConversationTab::write_items(&mut writer, buffer, self.selected_thread, &mut review.threads.iter(), |item, writer, buffer| {
                        ConversationTab::write_comment(writer, buffer, item.comments.get(0).unwrap()); //TODO remove unwrap
                    });
                }
            },
            View::Comment => {
                if let ConversationItem::Review(review) = selected_conversation {
                    if let Some(thread) = review.threads.get(self.selected_thread as usize) { // "as usize" is hacky, but should work in most cases...
                        ConversationTab::write_items(&mut writer, buffer, -1, &mut thread.comments.iter(), |item, writer, buffer| {
                            ConversationTab::write_comment(writer, buffer, item);
                        });
                    }
                }
            },
        }

        conversation_screen.draw_border(buffer);
        buffer.flush().unwrap();
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
