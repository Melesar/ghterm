use std::io::Write;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub fn draw_screen<W: Write>  (buffer: &mut W, rect: Rect, contents: &str) {
    write!(buffer, "{}", termion::cursor::Goto(1,1)).unwrap();
    for row in 0..rect.h {
        for column in 0..rect.w {
            if row == 0 && column == 0 {
                write!(buffer, "┏").unwrap();
            } else if row == 0 && column == rect.w - 1 {
                write!(buffer, "┓").unwrap();
            } else if column == 0 && row == rect.h - 1 {
                write!(buffer, "┗").unwrap();
            } else if column == rect.w - 1 && row == rect.h - 1 {
                write!(buffer, "┛").unwrap();
            } else if row == 0 || row == rect.h - 1  {
                write!(buffer, "━").unwrap();
            } else if  column == 0 || column == rect.w - 1 {
                write!(buffer, "┃").unwrap();
            } else { 
                write!(buffer, " ").unwrap();
            }
        }
    }

    write!(buffer, "{}Hello!{}", 
           termion::cursor::Goto(2,2),
           termion::cursor::Goto(rect.w, rect.h))
       .unwrap();
}
