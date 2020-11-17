use ncurses as c;
use std::env;
use std::fs;
use std::{thread::sleep, time::Duration};

fn main() {
    //let args: Vec<_> = env::args().collect();
    let mut buffer: Vec<Vec<c::chtype>> = vec![Vec::new()];
    let mut lines: usize = 1;
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
            match ch {
                c::KEY_END => {
                    break;
                }
                c::KEY_UP => {
                    if ln > 0 {
                        ln -= 1;
                        cl = buffer[ln].len();
                    }
                }
                c::KEY_DOWN => {
                    ln += 1;
                    if ln >= lines {
                        buffer.push(Vec::new());
                        lines = buffer.len();
                    }
                    cl = buffer[ln].len();
                }
                c::KEY_LEFT => {
                    if cl > 0 {
                        cl -= 1;
                    }
                }
                c::KEY_RIGHT => {
                    if cl < buffer[ln].len() {
                        cl += 1;
                    }
                }
                _ => {}
            }
        }
        for line in buffer.iter() {
            for &ch in line {
                c::addch(ch);
            }
            c::mv(c::getcury(w) + 1, 0);
        }
        c::mv(ln as i32, cl as i32);
        c::refresh();
        sleep(Duration::from_millis(20));
        c::clear();
    }
    c::endwin();
    for line in buffer.iter() {
        for ch in line {
            print!("{}", *ch as u8 as char);
        }
        println!("");
    }
}
