use crate::backend::pr::*;
use std::io::Write;
use crate::frontend::screen::ScreenWriter;
use super::conversation_draw::ConversationDraw;
use if_chain::if_chain;

pub struct ConversationTree {
    conversation: PrConversation,
    nodes: Vec<ConversationTreeNode>,
    selected_node: usize,
}

struct ConversationTreeNode {
    data: ConversationTreeItem,
    is_selected: bool,
    is_expanded: bool,

    next: Option<usize>,
    previous: Option<usize>,
    parent: Option<usize>,
    child: Option<usize>,
}

struct ConversationTreeItem(usize, Option<usize>);

impl ConversationTree {

    pub fn new (conversation: PrConversation) -> Self {
        let mut nodes = vec![];
        let mut previous = None;

        for (index, item) in conversation.items.iter().enumerate() {
            match item {

                ConversationItem::Review(r) => {
                    let review_node = ConversationTreeNode {
                        data: ConversationTreeItem(index, None),
                        next: if index != conversation.items.len() - 1 { Some(nodes.len() + r.threads.len() + 1) } else { None },
                        previous,
                        parent: None,
                        child: if r.threads.len() > 0 { Some(nodes.len() + 1) } else { None },

                        is_selected: index == 0,
                        is_expanded: true,
                    };

                    let review_node_index = nodes.len();
                    nodes.push(review_node);
                    previous = Some(review_node_index);

                    for (thread_index, _) in r.threads.iter().enumerate() {
                        let thread_node = ConversationTreeNode {
                            data: ConversationTreeItem(index, Some(thread_index)),
                            next: if thread_index != r.threads.len() - 1 { Some(nodes.len() + 1) } else { None },
                            previous,
                            parent: Some(review_node_index),
                            child: None,

                            is_selected: false,
                            is_expanded: false,
                        };
                        nodes.push(thread_node);
                        previous = Some(nodes.len() - 1);
                    }
                },

                ConversationItem::Comment(_) => {
                    let comment_node = ConversationTreeNode {
                        data: ConversationTreeItem(index, None),
                        next: if index != conversation.items.len() - 1 { Some(nodes.len() + 1) } else { None },
                        previous,
                        parent: None,
                        child: None,

                        is_selected: index == 0,
                        is_expanded: false,
                    };

                    previous = Some(nodes.len());
                    nodes.push(comment_node);
                },
            }
        }

        ConversationTree { conversation, nodes, selected_node: 0 }
    }

    pub fn move_selection (&self, forward: bool) {

    }

    pub fn draw (&self, buffer: &mut dyn Write, writer: &mut ScreenWriter) {
        let mut node_index = 0;
        while let Some(current_node) = self.nodes.get(node_index) {
            let draw = self.get_draw(&current_node.data);
            let is_selected = node_index == self.selected_node;

            writer.set_selection(is_selected);
            draw.draw(buffer, writer, current_node.is_expanded);
            writer.set_selection(false);

            if let Some(child_index) = current_node.child {
                node_index = child_index;
            } else if let Some(next_index) = current_node.next {
                node_index = next_index;
            } else { 
                if_chain! {
                    if let Some(parent_index) = current_node.parent;
                    if let Some(next_sibling_node) = self.nodes.get(parent_index);
                    if let Some(next_sibling_index) = next_sibling_node.next;
                    then {
                            node_index = next_sibling_index;
                            writer.write_line(buffer, "");
                    } else {
                            node_index = self.nodes.len();
                    }
                } 
            }
        }
    }

    fn get_draw<'a>(&'a self, index: &ConversationTreeItem) -> &'a dyn ConversationDraw {
        let (conversation_index, thread_index) = (index.0, index.1);
        let conversation_item = self.conversation.items.get(conversation_index).unwrap();
        match conversation_item {
            ConversationItem::Review(r) => {
                if let Some(thread_index) = thread_index {
                    r.threads.get(thread_index).unwrap()
                } else {
                    r
                }
            },
            ConversationItem::Comment(c) => c,
        }
    }
}
