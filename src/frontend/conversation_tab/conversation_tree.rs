use tui::{
    widgets::{Widget, StatefulWidget, Block, Borders},
    style::{Style},
    layout::{Rect},
    buffer::Buffer,
};

#[derive(Copy, Clone)]
pub struct Prefixes<'a> {
    pub collapsed_symbol: &'a str,
    pub expanded_symbol: &'a str,
    pub comment_prefix: &'a str,
}

impl<'a> Prefixes<'a> {
    pub fn new (collapsed_symbol: &'a str, expanded_symbol: &'a str, comment_prefix: &'a str) -> Self {
        Prefixes { collapsed_symbol, expanded_symbol, comment_prefix }
    }
}

impl<'a> Default for Prefixes<'a> {

    fn default() -> Self {
        Prefixes {
            collapsed_symbol: "",
            expanded_symbol: "",
            comment_prefix: "",
        }
    }
}

pub struct ConversationTree<'a> {
    block: Block<'a>,
    style: Style,
    highlighted_style: Style,
    prefixes: Prefixes<'a>,
}

impl<'a> ConversationTree<'a> {
    pub fn block(mut self, b: Block<'a>) -> Self {
        self.block = b;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlighted_style(mut self, style: Style) -> Self {
        self.highlighted_style = style;
        self
    }

    pub fn prefixes(mut self, prefixes: Prefixes<'a>) -> Self {
        self.prefixes = prefixes;
        self
    }
}

impl<'a> Default for ConversationTree<'a> {
    fn default () -> Self {
        ConversationTree { 
            block: Block::default(),
            style: Style::default(),
            highlighted_style: Style::default(),
            prefixes: Prefixes::default(),
        }
    }
}

impl<'a> StatefulWidget for ConversationTree<'a> {
    type State = Option<super::ConversationTreeState>;
    
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);

        let tree_area = self.block.inner(area);
        self.block.render(area, buf);

        if state.is_none() || tree_area.width < 1 || tree_area.height < 1 { return; }

        let state = state.as_ref().unwrap();
        let mut node_index = 0;
        let mut y_offset = 0_u16;
        while let Some(current_node) = state.nodes.get(node_index) {
            let draw = state.get_tree_draw(&current_node.data);
            let is_selected = node_index == state.selected_node;

            let style = if is_selected { self.highlighted_style } else { self.style };
            let area = Rect::new(tree_area.x, tree_area.y + y_offset, tree_area.width, 1);
            draw.draw(area, buf, self.prefixes, style, current_node.is_expanded);
            y_offset += 1;

            node_index = if let Some(child_index) = current_node.child {
                if current_node.is_expanded {
                    child_index
                } else if let Some(next_index) = current_node.next {
                    next_index
                } else {
                    state.nodes.len()
                }
            } else if let Some(next_index) = current_node.next {
                next_index
            } else { 
                current_node.parent
                    .and_then(|parent_index| state.nodes.get(parent_index))
                    .and_then(|next_sibling_node| next_sibling_node.next)
                    .unwrap_or(state.nodes.len())
            }
        }
    }
}
