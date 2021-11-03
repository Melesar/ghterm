use std::sync::mpsc;
use std::ops::DerefMut;

use super::screen::*;

use super::util::*;
use tui::{
    backend::Backend,
    style::{Style, Modifier},
    widgets::{Block, Borders, ListItem, List},
    layout::{Layout, Direction, Constraint},
    Frame,
};

use crate::backend::pr::PrHeader;
use crate::app::events::AppEvent;

use termion::event::Key;

pub struct RepoSelectionScreen  {
    event_sender: mpsc::Sender<AppEvent>,
    prs: StatefulList<PrHeader>,
}

impl RepoSelectionScreen {
    pub fn new(event_sender: mpsc::Sender<AppEvent>) -> Self {
        RepoSelectionScreen { event_sender, prs: StatefulList::new() }
    }

    pub fn set_pr_list(&mut self, prs: Vec<PrHeader>) {
        self.prs.items.extend(prs);
        if !self.prs.items.is_empty() {
            self.prs.select(0);
            self.event_sender.send(AppEvent::ScreenRepaint).unwrap();
        }
    }

    fn update_selection(&mut self, delta: i32) {
        if delta == 0 || self.prs.items.is_empty() { return; }

        if delta > 0 {
            self.prs.next();
        } else {
            self.prs.previous();
        }

        self.event_sender.send(AppEvent::ScreenRepaint).unwrap();
    }
}

impl<B: Backend> DrawableScreen<B> for RepoSelectionScreen  {

    fn draw (&self, frame: &mut Frame<B>) {

        let prs = &self.prs.items;
        let size = frame.size();
        let box_height = if !prs.is_empty() { prs.len().min(6) as u16 + 2 } else { 3 };
        let margins_size = (size.height - box_height) / 2;
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(margins_size),
                    Constraint::Length(box_height),
                    Constraint::Length(margins_size),
                ]
                .as_ref(),
            )
            .split(size);

        let horizontal_percentage = 80;
        let horizontal_margin_percentage = (100 - horizontal_percentage) / 2;
        let popup_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(horizontal_margin_percentage),
                    Constraint::Percentage(horizontal_percentage),
                    Constraint::Percentage(horizontal_margin_percentage),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1];

        let list_items : Vec<ListItem> = prs
                .iter()
                .map(|pr| ListItem::new(format!("#{} {}", pr.number, pr.title)))
                .collect();

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Select pull request"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, popup_layout, self.prs.state.borrow_mut().deref_mut());
    }
}

impl InteractableScreen for RepoSelectionScreen {
    fn validate_input(&self, input: Key) -> bool {
        if self.prs.items.is_empty() {
            return false;
        }

        match input {
            Key::Char('j') | Key::Char('k') | Key::Char('\n') => true,
            _ => false,
        }
    }

    fn process_input(&mut self, input: Key) {
        match input {
            Key::Char('j') => self.update_selection(1), 
            Key::Char('k') => self.update_selection(-1),
            Key::Char('\n') => 
                if let Some(chosen_repo) = self.prs.get_selected() {
                    self.event_sender.send(AppEvent::RepoChosen(chosen_repo.number)).unwrap();
                },
            _ => (),
        }
    }

}
