mod conversation_tree;
mod conversation_draw;

use std::rc::Rc;
use crate::backend::diff::ChangeList;
use std::sync::mpsc;

use crate::backend::pr::PrConversation;

use super::main_screen_handler::MainScreenEvent;
use super::screen::*;
use conversation_tree::ConversationTree;
use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;

pub struct ConversationTab {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation_tree: Option<ConversationTree>,
    changelist: Option<Rc<ChangeList>>,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab { screen_event_sender, conversation_tree: None, changelist: None }
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation_tree = Some(ConversationTree::new(conversation));
    }

    pub fn set_changelist(&mut self, changelist: Rc<ChangeList>) {
        self.changelist = Some(Rc::clone(&changelist));
    }
}

impl<B: Backend> DrawableScreen<B> for ConversationTab {

    fn draw(&self, frame: &mut Frame<B>) {
        /*let mut left_part = rect.screen();
        let mut right_part = left_part.split_vertically();
        let mut writer = left_part.get_content_rect().screen().get_writer();

        write!(buffer, "{}{}", termion::cursor::Goto(rect.x + 1, rect.y + 1), termion::clear::AfterCursor).unwrap();

        if let Some(conversation_tree) = self.conversation_tree.as_ref() {
            conversation_tree.draw_tree(buffer, &mut writer);
            conversation_tree.draw_selected_item(buffer, &mut right_part, &self.changelist);
        }

        left_part.draw_border(buffer);
        right_part.draw_border(buffer);*/
    }
}

impl InteractableScreen for ConversationTab {
    fn validate_input(&self, input: Key) -> bool {
        self.conversation_tree.is_some() &&
            (input == Key::Char('j') || input == Key::Char('k') || input == Key::Char('h') || input == Key::Char('l') ||
             input == Key::Char(' '))
    }

    fn process_input(&mut self, input: Key) {
        let vertical_offset = match input {
            Key::Char('j') => 1,
            Key::Char('k') => -1,
            _ => 0
        };
        let horizontal_offset = match input {
            Key::Char('h') => -1, 
            Key::Char('l') => 1,
            _ => 0,
        };
        
        let tree = self.conversation_tree.as_mut().unwrap();
        if input == Key::Char(' ') {
            tree.toggle_expansion();
        } else if vertical_offset != 0 {
            tree.move_selection(vertical_offset > 0);
        } 
    }
}

impl<B: Backend> ApplicationScreen<B> for ConversationTab {
}
