use ncurses as c;
use std::env;
use std::fs;

fn main() {
    //let args: Vec<_> = env::args().collect();
    // initialize window
    c::initscr();
    let w = c::stdscr();
    c::keypad(w, true);
    c::noecho();
    let mut maxy = 0;
    let mut maxx = 0;
    c::getmaxyx(w, &mut maxy, &mut maxx);
    c::endwin();
}
