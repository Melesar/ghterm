use crate::app::events::AppEvent;
use crate::backend::pr;
use super::screen::*;

use std::sync::mpsc;
use std::io::Write;
use std::marker::PhantomData;

use super::screen::{DrawableScreen, InteractableScreen};
use super::conversation_tab::ConversationTab;

pub trait MainScreenTab<W: Write> : ApplicationScreen<W> {
    fn get_title(&self) -> String;
}

pub struct MainScreen <W: Write> {
    tabs: Vec<Box<dyn MainScreenTab<W>>>,
    current_tab_index: usize,
    event_sender: mpsc::Sender<AppEvent>,
    pr: Option<pr::Pr>,
    _marker: PhantomData<W>,
}

impl <W: Write> MainScreen<W> {
    pub fn new (event_sender: mpsc::Sender<AppEvent>) -> Self {
        MainScreen{tabs: vec![
        ], current_tab_index: 0, event_sender, pr: None, _marker: PhantomData}
    }

    pub fn set_pr(&mut self, pr: pr::Pr) {
        self.pr = Some(pr);
    }
}

impl <W: Write> DrawableScreen <W> for MainScreen <W> {
    fn draw(&self, buffer: &mut W, rect: Rect) {
        let rect = Rect {y: rect.y + 1, h: rect.h - 1, ..rect};
        let screen = Screen::new(rect);
        screen.draw_border(buffer);
        buffer.flush().unwrap();
    }
}

impl <W: Write> InteractableScreen for MainScreen <W> {
    fn validate_input(&self, input: u8) -> bool {
        false
    }

    fn process_input(&mut self, input: u8) {
        
    }
}
