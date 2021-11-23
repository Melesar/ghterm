use crate::frontend::conversation_tab::ChangeList;
use crate::frontend::conversation_tab::conversation_tree::Prefixes;
use crate::backend::pr::*;
use std::rc::Rc;

use tui::{
    layout::{Rect, Layout, Direction, Constraint},
    buffer::Buffer,
    style::{Style, Modifier},
    text::{Span, Spans},
    widgets::{Widget, Paragraph, Wrap, Block, Borders},
};

pub trait TreeDraw {
    fn draw(&self, area: Rect, buffer: &mut Buffer, prefixes: Prefixes, style: Style, is_expanded: bool);
}

pub trait ContentDraw {
    fn draw(&self, area: Rect, buffer: &mut Buffer, style: Style, changelist: &Option<Rc<ChangeList>>);
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
    fn draw(&self, area: Rect, buffer: &mut Buffer, style: Style, _: &Option<Rc<ChangeList>>) {
        let text = vec![
            Spans::from(Span::raw(format!("{} {}", self.review_comment.author_name, self.verdict))),
            Spans::from(Span::raw(&self.review_comment.body)),
        ];
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::all()))
            .style(style);
        paragraph.render(area, buffer);
    }
}

impl ContentDraw for PrComment {
    fn draw(&self, area: Rect, buffer: &mut Buffer, style: Style, _: &Option<Rc<ChangeList>>) {
        let paragraph = Paragraph::new(self.body.as_str())
            .style(style)
            .block(Block::default().borders(Borders::all()));

        paragraph.render(area, buffer);
    }
}

impl ContentDraw for PrConversationThread {
    fn draw(&self, area: Rect, buffer: &mut Buffer, style: Style, changelist: &Option<Rc<ChangeList>>) {
        let mut threads_text = vec![];
        for comment in self.comments.iter() {
            threads_text.push(Spans::from(Span::styled(&comment.author_name, Style::default().add_modifier(Modifier::BOLD))));
            threads_text.push(Spans::from(Span::raw(&comment.body)));
            threads_text.push(Spans::from(Span::raw("")));
        }

        let paragraph = Paragraph::new(threads_text)
            .block(Block::default().borders(Borders::all()));

        if let Some(code_range) = &self.code_range {
            let hunk = changelist.as_ref().map(|c| c.get_hunk(code_range));
            let hunk_height = hunk.map_or(3, |h| std::cmp::min(h.lines().count() as u16, area.height / 2) + 2);
            let comments_height = area.height - hunk_height;
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(hunk_height), Constraint::Length(comments_height)]);

            let parts = layout.split(area);
            if let Some(hunk) = hunk {
                let diff_paragraph = Paragraph::new(hunk)
                    .wrap(Wrap{ trim: false })
                    .block(Block::default().borders(Borders::all()));
                diff_paragraph.render(parts[0], buffer);
            }
            paragraph.render(parts[1], buffer);
        } else {
            paragraph.render(area, buffer);
        }
/*
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
        }*/
    }
}
