use crate::collatz::*;
use colored::{ColoredString, Colorize};
use termion::{event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}, screen::{AlternateScreen, IntoAlternateScreen}};
use std::{io::{stdin, stdout, Write}, os::fd::AsFd};

pub fn run_program() -> std::io::Result<()> {
    
    let mut tree = CollatzTree::default();
    let mut from = 0;
    let mut newrows = 0isize;

    let stdin = stdin();
    let mut screen = stdout().into_raw_mode()?.into_alternate_screen()?;
    let _ = write!(screen, "{}", termion::cursor::Hide);
    write_head(&mut screen);
    write_tree(&mut screen, &tree, from, newrows);

    for k in stdin.keys() {
        
        // std keyset
        if from == 0 {
            match k {
                Ok(Key::Char('q')) => {break;}
                Ok(Key::Up) => { tree.expand_auto(); newrows += 1; }
                Ok(Key::Right) => { tree.expand_right(); newrows += 1; }
                Ok(Key::Left) => { tree.expand_left(); newrows += 1; }
                Ok(Key::Down) => { from += 1; newrows = 0; }
                _ => {}
            }
        }
        // alt keyset
        else {
            match k {
                Ok(Key::Char('q')) => {break;}
                Ok(Key::Up) => { from -= 1; newrows = 0; }
                Ok(Key::Right) => { from -= 1; newrows = 0; }
                Ok(Key::Left) => { from -= 1; newrows = 0; }
                Ok(Key::Down) => { from += 1; newrows = 0; }
                _ => {}
            }
        }

        write_head(&mut screen);
        write_tree(&mut screen, &tree, from, newrows);    

        let _ = screen.flush().unwrap();
    }
    let _ = write!(screen, "{}", termion::cursor::Show);

    Ok(())
}

fn write_head<W: std::io::Write + AsFd>(screen: &mut AlternateScreen<RawTerminal<W>>) {
    let _ = write!(screen, "{}{}{}\n\n{}", 
        termion::clear::All, 
        termion::cursor::Goto(1,1),
        "Collatz Viewer 1.0".black().on_white(),
        termion::cursor::Goto(1,3),
    );
    let _ = screen.flush();
}

fn write_tree<W: std::io::Write + AsFd>(screen: &mut AlternateScreen<RawTerminal<W>>, tree: &CollatzTree, from: usize, newrows: isize) {
    let offset = 3;
    let mut pos = 0;

    for item in tree.numlist.iter().rev().skip(from).take(termion::terminal_size().unwrap().1 as usize - 2) {
        let _ = write!(screen, "{}{} {}",
            termion::cursor::Goto(1, pos as u16 + offset as u16),
            if pos < newrows {" ".on_bright_yellow()} else {" ".on_bright_green()},
            item.to_string()
        );
        pos += 1;
    }

    if pos + offset <= termion::terminal_size().unwrap().1 as isize {
        let _ = write!(screen, "{}0",
            termion::cursor::Goto(1, pos as u16 + offset as u16)
        );
    }
    
    let _ = screen.flush();
}

impl ToString for CollatzTree {
    fn to_string(&self) -> String {
        let mut rstring = String::new();

        for item in self.numlist.iter().rev() {
            let istring = item.to_string();

            if item.val % 3 == 0 {
                rstring = format!("{rstring}\n{}", istring.red())
            } else {
                rstring = format!("{rstring}\n{istring}")
            }
        }

        rstring
    }
}

impl ToString for TreeItem {
    fn to_string(&self) -> String {
        let mut rstring = String::new();
        for _ in 0..self.xpos { rstring.push_str("│   "); }

        rstring = format!("{rstring}{}", fmt_num(self.val));
        rstring
    }
}

fn fmt_num(num: u128) -> ColoredString {
    let numstr = ColoredString::from(format!("├───{num}"));
    
    if num % 3 == 0 {
        numstr.red()
    } else {
        numstr
    }
}