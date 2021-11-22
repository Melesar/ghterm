
use tui::{
    widgets::{StatefulWidget},
    layout::{Rect},
    buffer::Buffer,
};

pub struct ConversationTreeContent {

}

impl Default for ConversationTreeContent {
    fn default() -> Self {
        ConversationTreeContent {  }
    }
}

impl StatefulWidget for ConversationTreeContent {
    type State = Option<super::ConversationTreeState>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        
    }
}
