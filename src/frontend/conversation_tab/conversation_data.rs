use std::io::Write;

use crate::backend::pr::{PrConversation, ConversationItem, PrReview, PrComment, PrConversationThread}; 
use crate::frontend::screen::ScreenWriter;

pub trait ConversationData {
    fn draw(&self, writer: &mut ScreenWriter, buffer: &mut dyn Write, conversation_idx: usize, thread_index: Option<usize>, comment_index: Option<usize>);
    fn try_move(&self, horizontal_offset: i32, vertical_offset: i32, conversation_idx: &mut usize, thread_index: &mut Option<usize>, comment_idx: &mut Option<usize>);
}

impl ConversationData for PrConversation {

    //TODO implement comments drawing
    fn draw(&self, writer: &mut ScreenWriter, buffer: &mut dyn Write, conversation_idx: usize, thread_index: Option<usize>, comment_index: Option<usize>) {
        
        let item = self.items.get(conversation_idx);
        if item.is_none() { return; }

        let item = item.unwrap();
        match item {
            ConversationItem::Review(review) => {
                if thread_index.is_none() {
                    write_review(writer, buffer, review);
                } else if review.threads.len() > 0 {
                    draw_threads(writer, buffer, &review.threads);
                }
            },

            ConversationItem::Comment(comment) => {
                write_comment(writer, buffer, comment);
            }
        }
    }

    fn try_move(&self, horizontal_offset: i32, vertical_offset: i32, conversation_idx: &mut usize, thread_index: &mut Option<usize>, comment_idx: &mut Option<usize>) {

        let conversation = self.items.get(*conversation_idx);
        if conversation.is_none() { return; }

        let conversation = conversation.unwrap();
        try_move_horizontally(horizontal_offset, &conversation, thread_index, comment_idx);

        if thread_index.is_none() {
            try_move_vertically(self.items.len(), vertical_offset, conversation_idx);
            return;
        }

        //At this point we are in threads column, so only reviews are relevant
        if let ConversationItem::Review(review) = conversation {
            let thread = thread_index.as_mut().unwrap();

            if let Some(comment) = comment_idx.as_mut() {
                let thread = review.threads.get(*thread).unwrap();
                try_move_vertically(thread.comments.len(), vertical_offset, comment);
                *comment_idx = Some(*comment);

            } else {
                try_move_vertically(review.threads.len(), vertical_offset, thread);
                *thread_index = Some(*thread);
            }

        }
    }
}

fn draw_threads(writer: &mut ScreenWriter, buffer: &mut dyn Write, threads: &Vec<PrConversationThread>) {
    
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

fn try_move_vertically(items_count: usize, offset: i32, index: &mut usize) {
    let mut index_tmp = *index as i32;
    index_tmp = (index_tmp + offset).max(0).min((items_count - 1) as i32);
    *index = index_tmp as usize;
}


fn try_move_horizontally(offset: i32, conversation: &ConversationItem, thread_idx: &mut Option<usize>, comment_idx: &mut Option<usize>) {
    if offset < 0 {
        if comment_idx.is_some() {
            *comment_idx = None;

        } else if thread_idx.is_some() {
            *comment_idx = None;
            *thread_idx = None;
        }

    } else if offset > 0 {
        if let ConversationItem::Review(review) = conversation {
            if thread_idx.is_none() {
                *thread_idx = if review.threads.is_empty() { None } else { Some(0) };

            } else if comment_idx.is_none() {
                *comment_idx = thread_idx.map_or(None,
                                     |i| review.threads.get(i).map_or(None,
                                          |t| if t.comments.len() > 0 {Some(0)} else {None}));
            }
        }
    }
}

