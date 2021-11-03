use std::io::Write;
use std::iter::FromIterator;
use termion::cursor::Goto;
use termion::event::Key;
use tui::backend::Backend;
use tui::Frame;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn screen(&self) -> Screen {
        Screen::new(*self)
    }
}

pub trait DrawableScreen<B: Backend>{
    fn draw(&self, frame: &mut Frame<B>);
}

pub trait InteractableScreen {
    fn validate_input(&self, input: Key) -> bool;
    fn process_input(&mut self, input: Key);
}

pub trait ApplicationScreen<B: Backend> : DrawableScreen<B> + InteractableScreen { 
}

pub trait ScreenHandler<B: Backend> : ApplicationScreen<B> {
    fn update (&mut self);
}

pub struct Screen {
    rect: Rect,
}

impl Screen {
    pub fn new(rect: Rect) -> Screen {
        Screen { rect }
    }

    pub fn draw_border(&self, buffer: &mut dyn Write) {
        if self.rect.h < 2 || self.rect.w < 2 {
            return;
        }
        
        let x = self.rect.x + 1;
        let y = self.rect.y + 1;

        write!(buffer, "{}{}", Goto(x,y), "┏").unwrap();
        write!(buffer, "{}{}", Goto(self.rect.x + self.rect.w, y), "┓").unwrap();
        write!(buffer, "{}{}", Goto(x, self.rect.y + self.rect.h), "┗").unwrap();
        write!(buffer, "{}{}", Goto(self.rect.x + self.rect.w, self.rect.y + self.rect.h), "┛").unwrap();

        for column in 1..self.rect.w - 1 {
            write!(buffer, "{}━", Goto(x + column, y)).unwrap();
            write!(buffer, "{}━", Goto(x + column, self.rect.y + self.rect.h)).unwrap();
        }

        for row in 1..self.rect.h - 1 {
            write!(buffer, "{}┃", Goto(self.rect.x + 1, y + row)).unwrap();
            write!(buffer, "{}┃", Goto(self.rect.x + self.rect.w, y + row)).unwrap();
        }
    }

    pub fn merge_vertically(top: Screen, bottom: Screen) -> Self {
        let rect = Rect {x: top.rect.x, y: top.rect.y, h: top.rect.h + bottom.rect.h, w: top.rect.w };
        Screen::new(rect)
    }

    pub fn merge_horizontally(left: Screen, right: Screen) -> Self {
        let rect = Rect {x: left.rect.x, y: left.rect.y, h: left.rect.h, w: left.rect.w + right.rect.w };
        Screen::new(rect)
    }

    pub fn split_vertically(&mut self) -> Self {
        let left = Rect { x: self.rect.x, y: self.rect.y, h: self.rect.h, w: self.rect.w / 2 };
        let right = Rect { x: self.rect.x + self.rect.w / 2, y: self.rect.y, h: self.rect.h, w: self.rect.w - left.w };
        self.rect = left;
        Screen::new(right)
    }

    pub fn split_horizontally(&mut self) -> Self {
        let top = Rect { x: self.rect.x, y: self.rect.y, h: self.rect.h / 2, w: self.rect.w };
        let bottom = Rect { x: self.rect.x, y: self.rect.y + self.rect.h / 2, h: self.rect.h - top.h, w: self.rect.w };
        self.rect = top;
        Screen::new(bottom)
    }

    pub fn get_writer(&self) -> ScreenWriter {
        ScreenWriter { rect: self.get_content_rect(), line_index: 0, is_selection: false, indent: 0}
    }

    pub fn get_full_rect(&self) -> Rect {
        self.rect
    }

    pub fn get_content_rect(&self) -> Rect {
        Rect {
            x: self.rect.x + 1,
            y: self.rect.y + 1,
            w: self.rect.w - 1,
            h: self.rect.h - 1,
        }
    }
}

pub struct ScreenWriter {
    rect: Rect,
    line_index: u16,
    is_selection: bool,
    indent: usize,
}

impl ScreenWriter {
    fn available_width(&self) -> u16 {
        self.rect.w - 2
    }

    fn left_padding(&self) -> u16 {
        self.rect.x + 2
    }

    fn draw_selection(&self, buffer: &mut dyn Write, y_pos: u16) {
        write!(buffer, "{}", Goto(self.rect.x, y_pos)).unwrap();
        if self.is_selection {
            write!(buffer, "{}", termion::color::Bg(termion::color::White)).unwrap();
        }
        write!(buffer, " {}", termion::color::Bg(termion::color::Reset)).unwrap();
    }

    pub fn write_line(&mut self, buffer: &mut dyn Write, message: &str) {
        let mut y_pos = self.rect.y + self.line_index + 1;
        if message.len() == 0 {
            self.draw_selection(buffer, y_pos);
            self.line_index += 1;
            return;
        }

        if self.rect.h <= self.line_index {
            return;
        }

        let available_width = self.available_width() - 8 * self.indent as u16;
        let mut total_characters = message.len();
        let mut characters_written : usize = 0;
        while total_characters > 0 {
            let mut to_write = std::cmp::min(available_width as usize, total_characters);
            let mut message_slice = &message[characters_written..(characters_written + to_write)];
            if let Some(idx) = message_slice.char_indices().find(|(_, c)| *c == '\n').map(|(i, _)| i) {
                message_slice = &message_slice[0..idx];
                to_write = idx + 1;
            }

            self.draw_selection(buffer, y_pos);
            write!(buffer, "{}", Goto(self.left_padding(), y_pos)).unwrap();
            write!(buffer, "{}{}", &String::from_iter(std::iter::repeat('\t').take(self.indent)), message_slice).unwrap();
            total_characters -= to_write;
            characters_written += to_write;
            self.line_index += 1;
            y_pos += 1;
        }
    }

    pub fn write_line_truncated(&mut self, buffer: &mut dyn Write, message: &str) {

        let y_pos = self.rect.y + self.line_index + 1;
        if message.len() == 0 {
            self.draw_selection(buffer, y_pos);
            self.line_index += 1;
            return;
        }

        if self.rect.h <= self.line_index {
            return;
        }

        let available_width = self.available_width() - 8 * self.indent as u16;
        let total_characters = message.len();
        let to_write = std::cmp::min(available_width as usize, total_characters);
        let mut message_slice = &message[0..to_write];

        if let Some(idx) = message_slice.char_indices().find(|(_, c)| *c == '\n').map(|(i, _)| i) {
            message_slice = &message_slice[0..idx];
        }

        self.draw_selection(buffer, y_pos);
        write!(buffer, "{}", Goto(self.left_padding(), y_pos)).unwrap();
        write!(buffer, "{}{}", &String::from_iter(std::iter::repeat('\t').take(self.indent)), message_slice).unwrap();
        self.line_index += 1;
    }

    pub fn set_selection(&mut self, is_selected: bool) {
        self.is_selection = is_selected;
    }

    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn separator(&mut self, buffer: &mut dyn Write) {
        if self.rect.h <= self.line_index {
            return;
        }
        
        let separator = String::from_iter(std::iter::repeat('-').take(self.available_width() as usize));
        write!(buffer, "{}{}", Goto(self.left_padding(), self.rect.y + self.line_index + 1), separator).unwrap();
        self.line_index += 1;
    }
}
