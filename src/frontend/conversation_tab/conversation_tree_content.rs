use std::rc::Rc;
use crate::backend::diff::ChangeList;
use super::ConversationTreeState;

use tui::{
    widgets::{Widget, Block},
    layout::Rect,
    style::Style,
    buffer::Buffer,
};

pub struct ConversationTreeContent<'a> {
    style: Style,
    block: Block<'a>,
    state: Option<&'a ConversationTreeState>,
    changelist: Option<Rc<ChangeList>>,
}

impl<'a> Default for ConversationTreeContent<'a> {
    fn default() -> Self {
        ConversationTreeContent { 
            style: Style::default(),
            block: Block::default(),
            state: None,
            changelist: None 
        }
    }
}

impl<'a> ConversationTreeContent<'a> {
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }

    pub fn state(mut self, state: Option<&'a ConversationTreeState>) -> Self {
        self.state = state;
        self
    }

    pub fn changelist(mut self, changelist: Option<Rc<ChangeList>>) -> Self {
        self.changelist = changelist;
        self
    }
}

impl<'a> Widget for ConversationTreeContent<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);

        if area.width < 1 || area.height < 1 { return; }

        if let Some(state) = self.state {
            state.draw_selected_item(area, buf, self.style, &self.changelist);
        }
    }
}
