use std::io::Write;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

#[derive(Hash, Eq, PartialEq)]
pub enum ScreenType {
    RepoSelection,
}

pub trait DrawableScreen <W: Write>{
    fn draw (&self, buffer: &mut W, rect: Rect);
}

pub trait InteractableScreen {
    fn validate_input(&self, input: u8) -> bool;


    /// This function should return true if as a result of 
    /// input processing the screen needs to be updated. Otherwise
    /// it should return false
    fn process_input(&mut self, input: u8) -> bool;
}

pub trait ApplicationScreen<W: Write> : DrawableScreen<W> + InteractableScreen { 
    fn screen_type(&self) -> ScreenType;
}

pub trait ScreenHandler<'a, W: Write> : InteractableScreen {
    fn update (&mut self, application_rect: Rect, force: bool);
}

pub struct Screen {
    rect: Rect,
}

impl Screen {
    pub fn new(rect: Rect) -> Screen {
        Screen { rect }
    }

    pub fn draw_border<W: Write>(&self, buffer: &mut W) {
        let x = self.rect.x + 1;
        let y = self.rect.y + 1;
        write!(buffer, "{}", termion::cursor::Goto(x,y)).unwrap();
        for row in 0..self.rect.h {
            for column in 0..self.rect.w {
                if row == 0 && column == 0 {
                    write!(buffer, "┏").unwrap();
                } else if row == 0 && column == self.rect.w - 1 {
                    write!(buffer, "┓").unwrap();
                } else if column == 0 && row == self.rect.h - 1 {
                    write!(buffer, "┗").unwrap();
                } else if column == self.rect.w - 1 && row == self.rect.h - 1 {
                    write!(buffer, "┛").unwrap();
                } else if row == 0 || row == self.rect.h - 1  {
                    write!(buffer, "━").unwrap();
                } else if  column == 0 || column == self.rect.w - 1 {
                    write!(buffer, "┃").unwrap();
                } else { 
                    write!(buffer, " ").unwrap();
                }
            }
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
