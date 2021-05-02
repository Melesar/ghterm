use std::io::Write;
use std::sync::mpsc;
use std::rc::Rc;

use super::screen::*;

use crate::backend::pr::PrHeader;
use crate::app::events::AppEvent;

pub struct RepoSelectionScreen {
    event_sender: mpsc::Sender<AppEvent>,
    prs: Option<Vec<PrHeader>>,
    selected_index: u32,
}

impl RepoSelectionScreen {
    pub fn new(event_sender: mpsc::Sender<AppEvent>) -> Self {
        RepoSelectionScreen {event_sender, prs: None, selected_index: 0}
    }

    pub fn set_pr_list(&mut self, prs: Vec<PrHeader>) {
        self.prs = Some(prs);
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
        if let Some(prs) = self.prs.as_ref() {
            crate::logs::log(&format!("Drawing {} prs", prs.len()));
            let rect = screen.get_content_rect();
            
            let mut start_position = (rect.x + 4, rect.h / 2);
            for pr in prs {
                write!(stdout, "{go}#{id}: {title}",
                       go = termion::cursor::Goto(start_position.0, start_position.1),
                       id = pr.number,
                       title = pr.title).unwrap();
                start_position.1 += 1;
            }
        }  

        stdout.flush().unwrap();
    }
}

impl InteractableScreen for RepoSelectionScreen {
    fn validate_input(&self, input: u8) -> bool {
        match input {
            //b'j' | b'k' | 13 => true,
            _ => false,
        }
    }

    fn process_input(&mut self, input: u8) {
        match input {
            b'j' => {
                self.selected_index += 1
            },
            b'k' => self.selected_index -= 1,
            13 => self.event_sender.send(AppEvent::RepoChosen(self.selected_index)).unwrap(),
            _ => (),
        }
    }
}


























