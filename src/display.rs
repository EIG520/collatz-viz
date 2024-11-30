use crate::collatz::*;
use colored::{ColoredString, Colorize};
use termion::{event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}, screen::{AlternateScreen, IntoAlternateScreen}};
use std::{io::{stdin, stdout, Write}, os::fd::AsFd};

pub fn run_program() -> std::io::Result<()> {
    
    let mut tree = CollatzTree::default();
    let mut from = 0;

    let stdin = stdin();
    let mut screen = stdout().into_raw_mode()?.into_alternate_screen()?;
    let _ = write!(screen, "{}", termion::cursor::Hide);
    write_head(&mut screen);
    write_tree(&mut screen, &tree, 0);

    for k in stdin.keys() {
        
        // std keyset
        if from == 0 {
            match k {
                Ok(Key::Char('q')) => {break;}
                Ok(Key::Up) => { tree.expand_auto(); }
                Ok(Key::Right) => { tree.expand_right(); }
                Ok(Key::Left) => { tree.expand_left(); }
                Ok(Key::Down) => { from += 1; }
                _ => {}
            }
        }
        // alt keyset
        else {
            match k {
                Ok(Key::Char('q')) => {break;}
                Ok(Key::Up) => { from -= 1; }
                Ok(Key::Right) => { from -= 1; }
                Ok(Key::Left) => { from -= 1; }
                Ok(Key::Down) => { from += 1; }
                _ => {}
            }
        }

        write_head(&mut screen);
        write_tree(&mut screen, &tree, from);    

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

fn write_tree<W: std::io::Write + AsFd>(screen: &mut AlternateScreen<RawTerminal<W>>, tree: &CollatzTree, from: usize) {
    let mut pos = 3;

    for item in tree.numlist.iter().rev().skip(from).take(termion::terminal_size().unwrap().1 as usize - 2) {
        let _ = write!(screen, "{}{}",
            termion::cursor::Goto(1, pos),
            item.to_string()
        );
        pos += 1;
    }

    if pos <= termion::terminal_size().unwrap().1 {
        let _ = write!(screen, "{}0",
            termion::cursor::Goto(1, pos)
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