use crate::app::events::AppEvent;
use crate::backend::pr::PrConversation;
use super::main_screen_handler::MainScreenEvent;
use super::screen::*;

use termion::cursor::Goto;

use std::sync::mpsc;
use std::io::Write;
use std::fmt::{Display, Formatter, Error};

use super::screen::{DrawableScreen, InteractableScreen};
use super::conversation_tab::ConversationTab;

pub enum MainScreenTab { 
    Conversation(ConversationTab),
    Timeline 
}

impl Display for MainScreenTab {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let label = match self {
            MainScreenTab::Conversation(_) => "Conversation",
            MainScreenTab::Timeline => "Timeline",
            _ => "",
        };
        write!(f, "{}", label)
    }
}

impl DrawableScreen for MainScreenTab {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        match self {
            MainScreenTab::Conversation(ct) => ct.draw(buffer, rect),
            _ => (),
        }
    }
}

impl InteractableScreen for MainScreenTab { 
    fn validate_input(&self, input: u8) -> bool {
        match self {
            MainScreenTab::Conversation(ct) => ct.validate_input(input),
            _ => false,
        }
    }

    fn process_input(&mut self, input: u8) {
        match self {
            MainScreenTab::Conversation(ct) => ct.process_input(input),
            _ => (),
        }
    }
}

pub struct MainScreen  {
    tabs: Vec<MainScreenTab>,
    current_tab_index: usize,
    app_event_sender: mpsc::Sender<AppEvent>,
}

impl MainScreen {
    pub fn new (app_event_sender: mpsc::Sender<AppEvent>, screen_event_sender: mpsc::Sender<MainScreenEvent>) -> Self {
        let tabs = vec![
            MainScreenTab::Conversation(ConversationTab::new(screen_event_sender.clone())),
        ];
        MainScreen{tabs, current_tab_index: 0, app_event_sender}
    }

    pub fn set_conversation(&mut self, conversation: PrConversation) {
        if let MainScreenTab::Conversation(ct) = &mut self.tabs[self.current_tab_index] {
            ct.set_conversation(conversation);
            self.app_event_sender.send(AppEvent::ScreenRepaint).unwrap();
        }
    }
}

impl DrawableScreen for MainScreen {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect) {
        let screen_rect = Rect {y: rect.y + 1, h: rect.h - 1, ..rect};
        let tab_screen = Screen::new(screen_rect);
        //tab_screen.draw_border(buffer);

        let mut title_offset : usize = 0;
        for (index, tab) in self.tabs.iter().enumerate() {
            let title = format!("{}. {}", index + 1, tab);
            write!(buffer, "{bg}{fg}{go}{title}{nobg}{nofg}",
                   //TODO change background color of not selected tabs
                   bg = termion::color::Bg(termion::color::LightBlack),
                   fg = termion::color::Fg(termion::color::White),
                   go = Goto(rect.x + title_offset as u16 + 1, rect.y + 1),
                   title = title,
                   nobg = termion::color::Bg(termion::color::Reset),
                   nofg = termion::color::Fg(termion::color::Reset)).unwrap();
            title_offset += title.len() + 1;
            if index == self.current_tab_index {
                tab.draw(buffer, tab_screen.get_full_rect());
            }
        }
        buffer.flush().unwrap();
    }
}

impl InteractableScreen for MainScreen {
    fn validate_input(&self, input: u8) -> bool {
        self.tabs[self.current_tab_index].validate_input(input)
    }

    fn process_input(&mut self, input: u8) {
        self.tabs[self.current_tab_index].process_input(input);
        self.app_event_sender.send(AppEvent::ScreenRepaint).unwrap();
    }
}
