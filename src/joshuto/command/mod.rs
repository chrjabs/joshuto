extern crate fs_extra;
extern crate ncurses;

use std;
use std::collections::HashMap;
use std::fmt;

use joshuto;

pub use self::quit::Quit;
mod quit;
pub use self::parent_directory::ParentDirectory;
mod parent_directory;
pub use self::open::Open;
mod open;
pub use self::cursor_move::CursorMove;
pub use self::cursor_move::CursorMovePageUp;
pub use self::cursor_move::CursorMovePageDown;
pub use self::cursor_move::CursorMoveHome;
pub use self::cursor_move::CursorMoveEnd;
mod cursor_move;

pub trait Runnable {
    fn execute(&self, context: &mut joshuto::JoshutoContext);
}

pub trait JoshutoCommand: Runnable + std::fmt::Display + std::fmt::Debug {}

#[derive(Debug)]
pub enum CommandKeybind {
    SimpleKeybind(Box<dyn JoshutoCommand>),
    CompositeKeybind(HashMap<i32, CommandKeybind>),
}

impl std::fmt::Display for CommandKeybind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandKeybind::SimpleKeybind(s) => write!(f, "{}", s),
            CommandKeybind::CompositeKeybind(_) => write!(f, "..."),
        }
    }
}

pub fn split_shell_style(line: &String) -> Vec<&str>
{
    let mut args: Vec<&str> = Vec::new();
    let mut char_ind = line.char_indices();

    while let Some((i, ch)) = char_ind.next() {
        if ch.is_whitespace() {
            continue;
        }
        let mut end_ind = i;
        if ch == '\'' {
            while let Some((j, ch)) = char_ind.next() {
                if ch == '\'' {
                    args.push(&line[i+1..j]);
                    break;
                }
            }
        } else if ch == '"'{
            while let Some((j, ch)) = char_ind.next() {
                if ch == '"' {
                    args.push(&line[i+1..j]);
                    break;
                }
            }
        } else {
            while let Some((j, ch)) = char_ind.next() {
                if ch.is_whitespace() {
                    args.push(&line[i..j]);
                    break;
                }
            }
        }
    }
    args
}

pub fn str_to_command()
{

}