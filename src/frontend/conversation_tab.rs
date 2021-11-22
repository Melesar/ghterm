mod conversation_tree;
mod conversation_tree_content;
mod conversation_tree_state;
mod conversation_draw;

use std::rc::Rc;
use std::sync::mpsc;
use std::cell::RefCell;
use std::ops::DerefMut;

use crate::backend::pr::PrConversation;
use crate::backend::diff::ChangeList;

use super::screen::InteractableScreen;
use super::main_screen_handler::MainScreenEvent;

use conversation_tree::{ConversationTree, Prefixes};
use conversation_tree_content::ConversationTreeContent;
use conversation_tree_state::ConversationTreeState;

use termion::event::Key;

use tui::{
    backend::Backend,
    layout::{Rect, Layout, Alignment, Direction, Constraint},
    widgets::{Block, Borders, BorderType},
    style::{Style, Modifier},
    Frame,
};

pub struct ConversationTab {
    screen_event_sender: mpsc::Sender<MainScreenEvent>,
    conversation_tree: RefCell<Option<ConversationTreeState>>,
    changelist: Option<Rc<ChangeList>>,
}

impl ConversationTab {
    pub fn new (screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        ConversationTab { screen_event_sender, conversation_tree: RefCell::new(None), changelist: None }
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        self.conversation_tree = RefCell::new(Some(ConversationTreeState::new(conversation)));
    }

    pub fn set_changelist(&mut self, changelist: Rc<ChangeList>) {
        self.changelist = Some(Rc::clone(&changelist));
    }

    pub fn draw<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(2, 3)
            ])
            .split(rect);

        let tree_widget = ConversationTree::default()
            .block(Block::default().borders(Borders::all()))
            .prefixes(Prefixes::new("▶", "▼", "-"))
            .highlighted_style(Style::default().add_modifier(Modifier::BOLD));
        frame.render_stateful_widget(tree_widget, layout[0], self.conversation_tree.borrow_mut().deref_mut());

        let state = self.conversation_tree.borrow();
        let content_widget = ConversationTreeContent::default()
            .block(Block::default().borders(Borders::all()))
            .state(state.as_ref())
            .changelist(self.changelist.as_ref().map(|rc| Rc::clone(rc)));
        frame.render_widget(content_widget, layout[1]);
    }
}

impl InteractableScreen for ConversationTab {
    fn validate_input(&self, input: Key) -> bool {
        self.conversation_tree.borrow().is_some() &&
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
        
        let mut borrow = self.conversation_tree.borrow_mut();
        (*borrow).as_mut().map(|t| {
            if input == Key::Char(' ') {
                t.toggle_expansion();
            } else if vertical_offset != 0 {
                t.move_selection(vertical_offset > 0);
            } 
        });
    }
}
