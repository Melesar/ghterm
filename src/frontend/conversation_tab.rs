use std::io::Write;
use std::sync::mpsc;

use crate::backend::pr::{ConversationItem, PrComment, PrConversation, PrReview, PrConversationThread};

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

enum View<'a> {
    None,
    Conversation(&'a Vec<ConversationItem>),
    Thread(&'a Vec<PrConversationThread>),
    Comment(&'a Vec<PrComment>),
}

pub struct ConversationTab<'a>  {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation: Option<PrConversation>,
    selected_conversation: usize,
    selected_thread: i32,
    current_view: View<'a>,
}

impl<'a> ConversationTab<'a> {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab { screen_event_sender, conversation: None, selected_conversation: 0, selected_thread: -1, current_view: View::None }
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation = Some(conversation);
        self.current_view = View::Conversation(&conversation.items);
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

    fn move_horizontally(&mut self, offset: i32) {
        match self.current_view {
            View::None => (),
            View::Conversation(items) => {
                
            },
        }
    }

    fn move_vertically(&mut self, offset: i32) {
        match self.current_view {
            View::Conversation(items) => {
                let mut current_index = self.selected_conversation as i32;
                current_index = (current_index + offset).max(0).min((items.len() - 1) as i32);
                self.selected_conversation = current_index as usize;
                self.selected_thread = -1;
            },
            View::Thread(threads) => {
            },
            _ => (),
        }
    }
}

impl<'a> DrawableScreen for ConversationTab<'a> {

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
            View::Conversation(items) => {
                ConversationTab::write_items(&mut writer, buffer, self.selected_conversation as i32, &mut items.iter(), |item, writer, buffer| {
                    match item {
                        ConversationItem::Review(r) => ConversationTab::write_review(writer, buffer, &r),
                        ConversationItem::Comment(c) => ConversationTab::write_comment(writer, buffer, c),
                    }
                });
            },
            View::Thread(threads) => {
                ConversationTab::write_items(&mut writer, buffer, self.selected_thread, &mut threads.iter(), |item, writer, buffer| {
                    ConversationTab::write_comment(writer, buffer, item.comments.get(0).unwrap()); //TODO remove unwrap
                });
            },
            View::Comment(comments) => {
                ConversationTab::write_items(&mut writer, buffer, -1, &mut comments.iter(), |item, writer, buffer| {
                    ConversationTab::write_comment(writer, buffer, item);
                });
            },
            View::None => (),
        }

        conversation_screen.draw_border(buffer);
        buffer.flush().unwrap();
    }
}

impl<'a> InteractableScreen for ConversationTab<'a> {
    fn validate_input(&self, input: u8) -> bool {
        self.conversation.is_some() && (input == b'j' || input == b'k' || input == b'h' || input == b'l')
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


        self.move_vertically(vertical_offset);
        self.move_horizontally(horizontal_offset);
    }
}

impl<'a> ApplicationScreen for ConversationTab<'a> {
}
