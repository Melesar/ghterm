use crate::frontend::screen::ScreenWriter;
use crate::backend::pr::*;
use std::io::Write;

pub trait ConversationDraw {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, is_expanded: bool);
}

impl ConversationDraw for PrReview {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, is_expanded: bool) {
        let has_threads = !self.threads.is_empty();
        let symbol = if has_threads && is_expanded { "▼" } else if has_threads { "▶" } else { " " };

        writer.write_line_truncated(buffer, &format!("{} {} {}", symbol, self.review_comment.author_name, self.verdict));
    }
}

impl ConversationDraw for PrComment {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, _: bool) {
        writer.write_line_truncated(buffer, &self.body);
    }
}


impl ConversationDraw for PrConversationThread {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, _: bool) {
        if let Some(comment) = self.comments.get(0) {
            writer.set_indent(1);
            writer.write_line_truncated(buffer, &format!("- {}", comment.body));
            writer.set_indent(0);
        }
    }
}
