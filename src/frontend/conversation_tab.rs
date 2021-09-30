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
    conversation: Option<PrConversation>,
    selected_conversation: usize,
    selected_thread: Option<usize>,
    selected_comment: Option<usize>,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab { screen_event_sender, conversation: None, selected_conversation: 0, selected_thread: None, selected_comment: None }
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation = Some(conversation);
    }
}

impl DrawableScreen for ConversationTab {

    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        let mut leftPart = rect.screen();
        let rightPart = leftPart.split_vertically();
        let mut writer = leftPart.get_content_rect().screen().get_writer();

        write!(buffer, "{}", termion::clear::All).unwrap();

       if self.conversation.is_none() {
           return;
       }

        let conversation = self.conversation.as_ref().unwrap();
        for item in conversation.items.iter() {
            match item {
                ConversationItem::Review(review) => {
                    writer.write_line_truncated(buffer, &format!("{} {}", review.review_comment.author_name, review.verdict));
                    writer.set_indent(1);
                    for thread in review.threads.iter() {
                        writer.write_line_truncated(buffer, &format!("{}", thread.comments[0].body));
                    }
                    writer.set_indent(0);
                },
                ConversationItem::Comment(comment) => {
                    writer.write_line(buffer, &format!("{} commented", comment.author_name));
                }
            }
        }

        leftPart.draw_border(buffer);
        rightPart.draw_border(buffer);
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

    }
}

impl ApplicationScreen for ConversationTab {
}

impl ConversationData for DummyConversation {
    fn draw(&self, _: &mut ScreenWriter, _: &mut dyn Write, _: usize, _: Option<usize>, _: Option<usize>) { }
    fn try_move(&self, _: i32, _: i32, _: &mut usize, _: &mut Option<usize>, _: &mut Option<usize>) { }
}
