use crate::Position;
//import io, io self, io Write
use std::io::{self, stdout, Write};
//import termion, a library for low-level handling, manipulating and reading information about terminals
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Size {
    pub width: u16,
    pub height: u16,
}
pub struct Terminal {
    size: Size,
    
    //set the terminal into raw mode (raw mode doesn't print out automatically)
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }

    //fn to launch clear terminal
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    //fn to place cursor on terminal
    #[allow(clippy::cast_possible_truncation)]            
    pub fn cursor_position(position: &Position) {
        let Position{mut x, mut y} = position;            
        x = x.saturating_add(1);            
        y = y.saturating_add(1);            
        let x = x as u16;            
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }

    //fn to ???
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    //fn to hide cursor
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    //fn to show cursor
    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    //fn to read key inputs on terminal
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}