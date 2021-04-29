use std::io::Write;
use std::sync::mpsc;

use super::screen::*;

use crate::backend;
use crate::app::events::AppEvent;

pub struct RepoSelectionScreen {
    event_sender: mpsc::Sender<AppEvent>,
}

impl<W: Write> ApplicationScreen<W> for RepoSelectionScreen {
    
    fn screen_type(&self) -> ScreenType {
        ScreenType::RepoSelection
    }
}

impl<W: Write> DrawableScreen<W> for  RepoSelectionScreen {

    fn draw (&self, stdout: &mut W, rect: Rect) {
        let screen = Screen::new(rect);
        screen.draw_border(stdout);
        let rect = screen.get_content_rect();
        
        let mut start_position = (rect.x + 4, rect.h / 2);
        let prs = backend::pr::list_prs().unwrap();
        for pr in prs {
            write!(stdout, "{go}#{id}: {title}",
                   go = termion::cursor::Goto(start_position.0, start_position.1),
                   id = pr.number,
                   title = pr.title).unwrap();
            start_position.1 += 1;
        }
    }
}

impl InteractableScreen for RepoSelectionScreen {
    fn validate_input(&self) -> bool {
        false
    }

    fn process_input(&mut self) {
        
    }
}


























