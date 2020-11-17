use ncurses as c;
use std::env;
use std::fs;

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
    c::noecho();
    let mut ch;
    loop {
        let mut maxy = 0;
        let mut maxx = 0;
        c::getmaxyx(w, &mut maxy, &mut maxx);

        c::mv(ln as i32, cl as i32);
        ch = c::getch();
        if 32 < ch && ch < 127 {
            // valid ASCII
            c::addch(ch as c::chtype);
            buffer[ln].push(ch as c::chtype);
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
                _ => {}
            }
        }
    }
    c::endwin();
}
