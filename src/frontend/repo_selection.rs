use std::io::Write;
use std::sync::mpsc;
use std::rc::Rc;

use super::screen::*;

use crate::backend::pr::PrsList;
use crate::app::events::AppEvent;

pub struct RepoSelectionScreen {
    event_sender: mpsc::Sender<AppEvent>,
    prs: Rc<PrsList>,
    selected_index: u32,
}

impl RepoSelectionScreen {
    
    pub fn new(event_sender: mpsc::Sender<AppEvent>, prs: Rc<PrsList>) -> Self {
        RepoSelectionScreen {event_sender, prs, selected_index: 0}
    }
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
        if let None = self.prs.get() {
            return;
        }
        let rect = screen.get_content_rect();
        
        let mut start_position = (rect.x + 4, rect.h / 2);
        let prs = crate::backend::pr::list_prs().unwrap();
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
    fn validate_input(&self, input: u8) -> bool {
        match input {
            b'j' | b'k' | 13 => true,
            _ => false,
        }
    }

    fn process_input(&mut self, input: u8) {
        match input {
            b'j' => self.selected_index += 1,
            b'k' => self.selected_index -= 1,
            13 => self.event_sender.send(AppEvent::RepoChosen(self.selected_index)).unwrap(),
            _ => (),
        }
    }
}


























