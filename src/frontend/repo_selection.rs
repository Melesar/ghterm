use std::io::{Write};
use super::screen;
use crate::backend;

pub struct RepoSelectionScreen {
}

impl screen::DrawableScreen for  RepoSelectionScreen {
    fn draw<W: Write>(stdout: &mut W, rect: screen::Rect) {
        let screen = screen::Screen::new(rect);
        screen.draw_border(stdout);
        let rect = screen.get_content_rect();
        
        let mut start_position = (rect.x + 4, rect.h / 2);
        let prs = backend::pr::PR::list().unwrap();
        for pr in prs {
            write!(stdout, "{go}#{id}: {title}",
                   go = termion::cursor::Goto(start_position.0, start_position.1),
                   id = pr.id,
                   title = pr.title).unwrap();
            start_position.1 += 1;
        }
    }
}
