use crate::frontend::conversation_tab::ChangeList;
use crate::backend::pr::*;
use std::io::Write;
use std::rc::Rc;
use crate::frontend::screen::{ScreenWriter, Screen};
use super::conversation_draw::{TreeDraw, ContentDraw};
use if_chain::if_chain;

pub struct ConversationTree {
    conversation: PrConversation,
    nodes: Vec<ConversationTreeNode>,
    selected_node: usize,
}

struct ConversationTreeNode {
    data: ConversationTreeItem,
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

                        is_expanded: r.threads.len() > 0,
                    };

                    let review_node_index = nodes.len();
                    nodes.push(review_node);
                    previous = Some(review_node_index);

                    for (thread_index, _) in r.threads.iter().enumerate() {
                        let thread_node = ConversationTreeNode {
                            data: ConversationTreeItem(index, Some(thread_index)),
                            next: if thread_index != r.threads.len() - 1 { Some(nodes.len() + 1) } else { None },
                            previous: Some(nodes.len() - 1),
                            parent: Some(review_node_index),
                            child: None,

                            is_expanded: false,
                        };
                        nodes.push(thread_node);
                    }
                },

                ConversationItem::Comment(_) => {
                    let comment_node = ConversationTreeNode {
                        data: ConversationTreeItem(index, None),
                        next: if index != conversation.items.len() - 1 { Some(nodes.len() + 1) } else { None },
                        previous,
                        parent: None,
                        child: None,

                        is_expanded: false,
                    };

                    previous = Some(nodes.len());
                    nodes.push(comment_node);
                },
            }
        }

        ConversationTree { conversation, nodes, selected_node: 0 }
    }

    pub fn move_selection (&mut self, forward: bool) {
        let selected_node = self.nodes.get(self.selected_node);
        if selected_node.is_none() {
            return;
        }

        let selected_node = selected_node.unwrap();
        self.selected_node = if forward {
            if selected_node.is_expanded {
                self.selected_node + 1
            } else if let Some(next_index) = selected_node.next {
                next_index
            } else if let Some(_) = selected_node.child { //The last review in the list and not expanded. Don't move
                self.selected_node
            } else {
                if self.nodes.len() > self.selected_node + 1 { self.selected_node + 1 } else { self.selected_node }
            }
        } else {
            if_chain! {
                if let Some(previous_index) = selected_node.previous;
                if let Some(previous_node) = self.nodes.get(previous_index);
                if let Some(_) = previous_node.child;
                if !previous_node.is_expanded;
                then {
                    previous_index
                } else {
                    if self.selected_node > 0 { self.selected_node - 1 } else { self.selected_node }
                }
            }
        }
    }

    pub fn toggle_expansion(&mut self) {
        if let Some(node) = self.nodes.get_mut(self.selected_node) {
            node.is_expanded = node.child.is_some() && !node.is_expanded;
        }
    }

    pub fn draw_tree (&self, buffer: &mut dyn Write, writer: &mut ScreenWriter) {
        let mut node_index = 0;
        while let Some(current_node) = self.nodes.get(node_index) {
            let draw = self.get_tree_draw(&current_node.data);
            let is_selected = node_index == self.selected_node;

            writer.set_selection(is_selected);
            draw.draw(buffer, writer, current_node.is_expanded);
            writer.set_selection(false);

            node_index = if let Some(child_index) = current_node.child {
                if current_node.is_expanded {
                    child_index
                } else if let Some(next_index) = current_node.next {
                    next_index
                } else {
                    self.nodes.len()
                }
            } else if let Some(next_index) = current_node.next {
                next_index
            } else { 
                if_chain! {
                    if let Some(parent_index) = current_node.parent;
                    if let Some(next_sibling_node) = self.nodes.get(parent_index);
                    if let Some(next_sibling_index) = next_sibling_node.next;
                    then {
                            next_sibling_index
                    } else {
                            self.nodes.len()
                    }
                } 
            }
        }
    }

    pub fn draw_selected_item(&self, buffer: &mut dyn Write, screen: &mut Screen, changelist: &Option<Rc<ChangeList>>) {
        if let Some(selected_node) = self.nodes.get(self.selected_node) {
            self.get_content_draw(&selected_node.data).draw(buffer, screen, changelist);
        }
    }

    //TODO figure out how to reuse code in these two methods
    fn get_tree_draw<'a>(&'a self, index: &ConversationTreeItem) -> &'a dyn TreeDraw {
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

    fn get_content_draw<'a>(&'a self, index: &ConversationTreeItem) -> &'a dyn ContentDraw {
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
