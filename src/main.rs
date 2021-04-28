use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::io::Write;

mod backend; 
mod frontend;

pub use backend::pr::PR;
pub use backend::gh;
pub use frontend::repo_selection::RepoSelectionScreen;

use frontend::screen::{Rect, DrawableScreen};

fn main() {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let stdin = std::io::stdin();
    
    write!(stdout, "{}{}",
           termion::clear::All,
           termion::cursor::Hide).unwrap(); 

    let size = termion::terminal_size().unwrap();
    let rect = Rect{x: 0, y: 0, w: size.0, h: size.1};

    RepoSelectionScreen::draw(&mut stdout, rect);
    stdout.flush().unwrap();
    
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            _ => continue,
        }
    }

    write!(stdout, "{}{}{}", termion::cursor::Restore, termion::cursor::Show, termion::clear::All).unwrap();
}
