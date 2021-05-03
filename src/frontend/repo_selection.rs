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

    fn update_selection(&mut self, delta: i32) {
        let prs = if let Some(prs) = self.prs.as_ref() {
            prs
        } else {
            return;
        };

        if prs.len() == 0 {
            return;
        }

        let mut current_index = self.selected_index as i32;
        let prs_count = prs.len() as i32;
        
        current_index += delta;
        current_index = if current_index < 0 {
            0
        } else if current_index >= prs_count {
            prs_count - 1
        } else {
            current_index
        };
        self.selected_index = current_index as u32;
    }
}

impl<W: Write> ApplicationScreen<W> for RepoSelectionScreen {
    
    fn screen_type(&self) -> ScreenType {
        ScreenType::RepoSelection
    }
}

impl<W: Write> DrawableScreen<W> for  RepoSelectionScreen {

    fn draw (&self, stdout: &mut W, rect: Rect) {
        crate::logs::log(&format!("Drawing repo selection screen"));
        let screen = Screen::new(rect);
        screen.draw_border(stdout);
        if let Some(prs) = self.prs.as_ref() {
            let rect = screen.get_content_rect();
            
            let mut start_position = (rect.x + 4, rect.h / 2);
            for (index, pr) in prs.iter().enumerate() {
                let is_selected  = index as u32 == self.selected_index;
                let bg : termion::color::Bg<&dyn termion::color::Color> = if is_selected {
                    termion::color::Bg(&termion::color::White)
                } else {
                    termion::color::Bg(&termion::color::Black)
                };
                let fg : termion::color::Fg<&dyn termion::color::Color> = if is_selected {
                    termion::color::Fg(&termion::color::Black)
                } else {
                    termion::color::Fg(&termion::color::White)
                };
                write!(stdout, "{go}{bg}{fg}#{id}: {title}{no_bg}{no_fg}",
                       go = termion::cursor::Goto(start_position.0, start_position.1),
                       id = pr.number,
                       title = pr.title,
                       bg = bg,
                       fg = fg,
                       no_bg = termion::color::Bg(termion::color::Reset),
                       no_fg = termion::color::Fg(termion::color::Reset),
                       ).unwrap();
                start_position.1 += 1;
            }
        }  

        stdout.flush().unwrap();
    }
}

impl InteractableScreen for RepoSelectionScreen {
    fn validate_input(&self, input: u8) -> bool {
        if self.prs.is_none() {
            return false;
        }

        match input {
            b'j' | b'k' | 13 => true,
            _ => false,
        }
    }

    fn process_input(&mut self, input: u8) -> bool {
        match input {
            b'j' => {
                self.update_selection(1);
                return true;
            },
            b'k' => { 
                self.update_selection(-1);
                return true;
            },
            13 => {
                self.event_sender.send(AppEvent::RepoChosen(self.selected_index)).unwrap();
                return false;
            }
            _ => false,
        }
    }

}


























