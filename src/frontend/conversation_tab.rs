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
        let conversation_screen = rect.screen();
        let mut writer = conversation_screen.get_content_rect().screen().get_writer();

        self.conversation.draw(&mut writer, buffer, self.selected_conversation, self.selected_thread, self.selected_comment);

        /*match self.current_view {
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
        }*/

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
