use ncurses as c;
use std::env;
use std::fs::*;
use std::io::prelude::*;
use std::{thread::sleep, time::Duration};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: `$ tet <filename>`");
        return;
    }
    let mut buffer: Vec<Vec<c::chtype>> = vec![Vec::new()];
    let mut ln: usize = 0;
    let mut cl: usize = 0;
    // initialize window
    c::initscr();
    let w = c::stdscr();
    c::keypad(w, true);
    c::nodelay(w, true);
    c::noecho();
    let mut ch;
    loop {
        let mut maxy = 0;
        let mut maxx = 0;
        c::getmaxyx(w, &mut maxy, &mut maxx);

        ch = c::getch();
        if 31 < ch && ch < 127 {
            // valid ASCII
            buffer[ln].insert(cl, ch as u32);
            cl += 1;
        } else {
            if !process_ch(ch, &mut buffer, &mut ln, &mut cl) {
                break;
            }
        }
        for line in buffer.iter() {
            for &ch in line {
                c::addch(ch);
            }
            c::mv(c::getcury(w) + 1, 0);
        }
        save(&args[1][..], &buffer);
        c::mv(ln as i32, cl as i32);
        c::refresh();
        sleep(Duration::from_millis(20));
        c::clear();
    }
    c::endwin();
}

fn process_ch(ch: i32, buffer: &mut Vec<Vec<u32>>, ln: &mut usize, cl: &mut usize) -> bool {
    match ch {
        c::KEY_END => {
            return false;
        }
        c::KEY_UP => {
            if *ln > 0 {
                *ln -= 1;
                if buffer[*ln].len() < *cl {
                    *cl = buffer[*ln].len();
                }
            }
        }
        c::KEY_DOWN => {
            *ln += 1;
            if *ln >= buffer.len() {
                buffer.push(Vec::new());
            }
            if buffer[*ln].len() < *cl {
                *cl = buffer[*ln].len();
            }
        }
        10 => {
            // ENTER
            *ln += 1;
            if *ln >= buffer.len() {
                buffer.push(Vec::new());
            }
            *cl = buffer[*ln].len();
        }
        9 => {
            for _ in 0..4 {
                buffer[*ln].push(' ' as u32);
                *cl += 1;
            }
        }
        c::KEY_LEFT => {
            if *cl > 0 {
                *cl -= 1;
            }
        }
        c::KEY_RIGHT => {
            if *cl < buffer[*ln].len() {
                *cl += 1;
            }
        }
        c::KEY_BACKSPACE => {
            if *cl > 0 {
                buffer[*ln].remove(*cl - 1);
                *cl -= 1;
            }
        }
        _ => {}
    }
    return true;
}

fn save(filename: &str, buffer: &Vec<Vec<u32>>) {
    let mut f = File::create(filename).unwrap();
    for line in buffer.iter() {
        for ch in line {
            f.write(&[*ch as u8]).unwrap();
        }
        f.write_all(b"\n").unwrap();
    }
}
