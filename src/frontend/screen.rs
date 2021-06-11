use std::io::Write;
use termion::cursor::Goto;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub trait DrawableScreen {
    fn draw(&self, buffer: &mut dyn Write, rect: Rect);
}

pub trait InteractableScreen {
    fn validate_input(&self, input: u8) -> bool;
    fn process_input(&mut self, input: u8);
}

pub trait ApplicationScreen : DrawableScreen + InteractableScreen { 
}

pub trait ScreenHandler : ApplicationScreen {
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

    pub fn split_vertically(self) -> (Self, Self) {
        let left = Rect { x: self.rect.x, y: self.rect.y, h: self.rect.h, w: self.rect.w / 2 };
        let right = Rect { x: self.rect.x + self.rect.w / 2, y: self.rect.y, h: self.rect.h, w: self.rect.w / 2 };
        (Screen::new(left), Screen::new(right))
    }

    pub fn split_horizontally(self) -> (Self, Self) {
        let top = Rect { x: self.rect.x, y: self.rect.y, h: self.rect.h / 2, w: self.rect.w };
        let bottom = Rect { x: self.rect.x, y: self.rect.y + self.rect.h / 2, h: self.rect.h / 2, w: self.rect.w };
        (Screen::new(top), Screen::new(bottom))
    }

    pub fn get_writer(&self) -> ScreenWriter {
        ScreenWriter { rect: self.get_content_rect(), line_index: 0 }
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
}

impl ScreenWriter {
    pub fn write_line(&mut self, buffer: &mut dyn Write, message: &str) {
        if message.len() == 0 {
            self.line_index += 1;
            return;
        }

        if self.rect.h <= self.line_index {
            return;
        }

        let available_width = self.rect.w;
        let mut total_characters = message.len();
        let mut characters_written : usize = 0;
        while total_characters > 0 {
            let y_pos = self.rect.y + self.line_index + 1;
            let mut to_write = std::cmp::min(available_width as usize, total_characters);
            let mut message_slice = &message[characters_written..(characters_written + to_write)];
            if let Some(idx) = message_slice.char_indices().find(|(_, c)| *c == '\n').map(|(i, _)| i) {
                message_slice = &message_slice[0..idx];
                to_write = idx + 1;
            }
            write!(buffer, "{}{}", Goto(self.rect.x, y_pos), message_slice).unwrap();
            total_characters -= to_write;
            characters_written += to_write;
            self.line_index += 1;
        }
    }
}
