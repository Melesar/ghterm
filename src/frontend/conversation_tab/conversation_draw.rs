use crate::frontend::conversation_tab::ChangeList;
use crate::frontend::screen::{ScreenWriter, Screen};
use crate::backend::pr::*;
use std::io::Write;
use std::rc::Rc;

pub trait TreeDraw {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, is_expanded: bool);
}

pub trait ContentDraw {
    fn draw(&self, buffer: &mut dyn Write, screen: &mut Screen, changelist: &Option<Rc<ChangeList>>);
}

impl TreeDraw for PrReview {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, is_expanded: bool) {
        let has_threads = !self.threads.is_empty();
        let symbol = if has_threads && is_expanded { "▼" } else if has_threads { "▶" } else { " " };

        writer.write_line_truncated(buffer, &format!("{} {} {}", symbol, self.review_comment.author_name, self.verdict));
    }
}

impl TreeDraw for PrComment {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, _: bool) {
        writer.write_line_truncated(buffer, &self.body);
    }
}


impl TreeDraw for PrConversationThread {
    fn draw(&self, buffer: &mut dyn Write, writer: &mut ScreenWriter, _: bool) {
        if let Some(comment) = self.comments.get(0) {
            writer.set_indent(1);
            writer.write_line_truncated(buffer, &format!("- {}", comment.body));
            writer.set_indent(0);
        }
    }
}

impl ContentDraw for PrReview {
    fn draw(&self, buffer: &mut dyn Write, screen: &mut Screen, changelist: &Option<Rc<ChangeList>>) {
        let mut writer = screen.get_content_rect().screen().get_writer();
        writer.write_line(buffer, &format!("{} {}", self.review_comment.author_name, self.verdict));
        writer.write_line(buffer, &self.review_comment.body);
    }
}

impl ContentDraw for PrComment {
    fn draw(&self, buffer: &mut dyn Write, screen: &mut Screen, changelist: &Option<Rc<ChangeList>>) {
        let mut writer = screen.get_content_rect().screen().get_writer();
        writer.write_line(buffer, &self.body);
    }
}

impl ContentDraw for PrConversationThread {
    fn draw(&self, buffer: &mut dyn Write, screen: &mut Screen, changelist: &Option<Rc<ChangeList>>) {
        let bottom_part = screen.split_horizontally();
        let mut thread_writer = bottom_part.get_content_rect().screen().get_writer();

        if let Some(changelist) = changelist.as_ref() {
            let mut changelist_writer = screen.get_content_rect().screen().get_writer();
            //changelist_writer.write_hunk(buffer, &self.code_hunk, changelist);
        }

        for comment in self.comments.iter() {
            thread_writer.write_line(buffer, &comment.author_name);
            thread_writer.write_line(buffer, &comment.body);
            thread_writer.write_line(buffer, "");
        }

        bottom_part.draw_border(buffer);
    }
}
