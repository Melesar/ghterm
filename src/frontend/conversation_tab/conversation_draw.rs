use crate::frontend::conversation_tab::ChangeList;
use crate::frontend::conversation_tab::conversation_tree::Prefixes;
use crate::frontend::screen::{ScreenWriter, Screen};
use crate::backend::pr::*;
use std::io::Write;
use std::rc::Rc;

use tui::{
    layout::Rect,
    buffer::Buffer,
    style::Style,
};

pub trait TreeDraw {
    fn draw(&self, area: Rect, buffer: &mut Buffer, prefixes: Prefixes, style: Style, is_expanded: bool);
}

pub trait ContentDraw {
    fn draw(&self, buffer: &mut dyn Write, screen: &mut Screen, changelist: &Option<Rc<ChangeList>>);
}

impl TreeDraw for PrReview {
    fn draw(&self, area: Rect, buffer: &mut Buffer, prefixes: Prefixes, style: Style, is_expanded: bool) {
        let has_threads = !self.threads.is_empty();
        let symbol = if has_threads && is_expanded { prefixes.expanded_symbol } else if has_threads { prefixes.collapsed_symbol } else { " " };

        buffer.set_stringn(area.x, area.y, format!("{} {} {}", symbol, self.review_comment.author_name, self.verdict), area.width as usize, style);
    }
}

impl TreeDraw for PrComment {
    fn draw(&self, area: Rect, buffer: &mut Buffer, _: Prefixes, style: Style, _: bool) {
        buffer.set_stringn(area.x, area.y, &self.body, area.width as usize, style);
    }
}


impl TreeDraw for PrConversationThread {
    fn draw(&self, area: Rect, buffer: &mut Buffer, prefixes: Prefixes, style: Style, _: bool) {
        if let Some(comment) = self.comments.get(0) {
            buffer.set_stringn(area.x + 4, area.y, format!("{} {}", prefixes.comment_prefix, comment.body), (area.width - 4) as usize, style);
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

        let mut thread_writer : ScreenWriter;

        if let Some(code_range) = &self.code_range {

            let thread_screen = screen.split_horizontally();
            thread_writer = thread_screen.get_content_rect().screen().get_writer();

            if let Some(changelist) = changelist.as_ref() {
                let mut changelist_writer = screen.get_content_rect().screen().get_writer();
                changelist_writer.write_line(buffer, changelist.get_hunk(code_range)); //TODO draw diff properly
            }

            thread_screen.draw_border(buffer);

        } else {
            thread_writer = screen.get_content_rect().screen().get_writer();
            screen.draw_border(buffer);
        }
 
        for comment in self.comments.iter() {
            thread_writer.write_line(buffer, &comment.author_name);
            thread_writer.write_line(buffer, &comment.body);
            thread_writer.write_line(buffer, "");
        }
    }
}
