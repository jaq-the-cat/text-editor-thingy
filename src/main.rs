use ncurses as c;
use std::env;
use std::fs;

fn main() {
    //let args: Vec<_> = env::args().collect();
    let mut buffer: Vec<Vec<char>> = Vec::new();
    let mut ln: usize = 0;
    let mut cl: usize = 0;
    // initialize window
    c::initscr();
    let w = c::stdscr();
    c::keypad(w, true);
    c::noecho();
    let mut ch = 0;
    loop {
        let mut maxy = 0;
        let mut maxx = 0;
        c::getmaxyx(w, &mut maxy, &mut maxx);
        ch = c::getch();
        if 32 < ch && ch < 127 {
            // valid ASCII
            c::addch(ch as c::chtype);
        }
    }
    c::endwin();
}
