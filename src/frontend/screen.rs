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

pub trait ScreenHandler  : ApplicationScreen {
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

    pub fn get_content_rect(&self) -> Rect {
        Rect {
            x: self.rect.x + 1,
            y: self.rect.y + 1,
            w: self.rect.w - 1,
            h: self.rect.h - 1,
        }
    }
}
