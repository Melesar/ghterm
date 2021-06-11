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

    pub fn get_content_rect(&self) -> Rect {
        Rect {
            x: self.rect.x + 1,
            y: self.rect.y + 1,
            w: self.rect.w - 1,
            h: self.rect.h - 1,
        }
    }
}
