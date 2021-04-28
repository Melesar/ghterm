use std::io::{Write};
use termion::clear;
use termion::cursor::Goto;

pub struct RepoSelectionScreen<W: Write> {
    stdout: W,
}

impl <W: Write> RepoSelectionScreen<W> {
    pub fn draw(mut stdout: W) {
        write!(stdout, "{}", clear::All).unwrap();
        let size = termion::terminal_size().unwrap();
        let rect = super::screen::Rect {x: 0, y: 0, w: size.0, h: size.1};
        super::screen::draw_screen(&mut stdout, rect, ""); 
    }
}
