use std::rc::Rc;
use crate::backend::diff::ChangeList;
use crate::app::events::AppEvent;
use crate::backend::pr::PrConversation;
use super::main_screen_handler::MainScreenEvent;
use tui::{
    backend::Backend,
    widgets::{Block, Borders, Tabs},
    style::{Style, Modifier, Color},
    text::{Spans, Span},
    layout::{Layout, Direction, Constraint},
    Frame,
};

use std::sync::mpsc;
use std::fmt::{Display, Formatter, Error};

use super::screen::{DrawableScreen, InteractableScreen};
use super::conversation_tab::ConversationTab;

pub enum MainScreenTab { 
    Conversation(ConversationTab),
}

impl Display for MainScreenTab {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let label = match self {
            MainScreenTab::Conversation(_) => "Conversation",
            _ => "",
        };
        write!(f, "{}", label)
    }
}

impl InteractableScreen for MainScreenTab { 
    fn validate_input(&self, input: termion::event::Key) -> bool {
        match self {
            MainScreenTab::Conversation(ct) => ct.validate_input(input),
            _ => false,
        }
    }

    fn process_input(&mut self, input: termion::event::Key) {
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
        for tab in self.tabs.iter_mut() {
            if let MainScreenTab::Conversation(ct) = tab {
                ct.set_conversation(conversation);
                self.app_event_sender.send(AppEvent::ScreenRepaint).unwrap();
                break;
            }
        }
    }

    pub fn set_changelist(&mut self, changelist: ChangeList) {
        let changelist = Rc::new(changelist);
        for tab in self.tabs.iter_mut() {
            if let MainScreenTab::Conversation(ct) = tab {
                ct.set_changelist(Rc::clone(&changelist));
                self.app_event_sender.send(AppEvent::ScreenRepaint).unwrap();
                break;
            }
        }
    }
}

impl<B: Backend> DrawableScreen<B> for MainScreen {
    fn draw(&self, frame: &mut Frame<B>) {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(size);

        let titles : Vec<Spans> = self.tabs.iter()
            .enumerate()
            .map(|(index, tab)| {
                Spans::from(format!("{} {}", index + 1, tab))
            })
            .collect();

        let tabs = Tabs::new(titles)
            .select(self.current_tab_index);

        frame.render_widget(tabs, layout[0]);

        if let Some(selected_tab) = self.tabs.get(self.current_tab_index) {
            match selected_tab {
                MainScreenTab::Conversation(ct) => ct.draw(frame, layout[1]),
            }
        }
    }
}

impl InteractableScreen for MainScreen {
    fn validate_input(&self, input: termion::event::Key) -> bool {
        self.tabs[self.current_tab_index].validate_input(input)
    }

    fn process_input(&mut self, input: termion::event::Key) {
        self.tabs[self.current_tab_index].process_input(input);
        self.app_event_sender.send(AppEvent::ScreenRepaint).unwrap();
    }
}
