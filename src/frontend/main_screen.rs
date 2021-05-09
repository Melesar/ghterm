use crate::app::events::AppEvent;
use crate::backend::pr;
use super::screen::*;

use std::sync::mpsc;
use std::collections::HashMap;
use std::io::Write;

use super::screen::{DrawableScreen, InteractableScreen};
use super::conversation_tab::ConversationTab;

pub struct MainScreen  {
    tabs: HashMap<String, Box<dyn ApplicationScreen>>,
    current_tab_index: usize,
    event_sender: mpsc::Sender<AppEvent>,
    pr: Option<pr::Pr>,
}

impl MainScreen {
    pub fn new (event_sender: mpsc::Sender<AppEvent>) -> Self {
        let mut tabs : HashMap<String, Box<dyn ApplicationScreen>> = HashMap::new();
        tabs.insert(String::from("1. Conversation"), Box::new(ConversationTab::new()));
        MainScreen{tabs, current_tab_index: 0, event_sender, pr: None}
    }

    pub fn set_pr(&mut self, pr: pr::Pr) {
        self.pr = Some(pr);
    }
}

impl  DrawableScreen  for MainScreen  {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        let screen_rect = Rect {y: rect.y + 1, h: rect.h - 1, ..rect};
        let screen = Screen::new(screen_rect);
        screen.draw_border(buffer);

        let mut title_offset : usize = 0;
        for (title, tab) in self.tabs {
            
        }
        buffer.flush().unwrap();
    }
}

impl  InteractableScreen for MainScreen  {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}
