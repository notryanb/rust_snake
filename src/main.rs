extern crate pancurses;

use pancurses::*;

fn main() {
    let window = initscr();
    start_color();
    use_default_colors();
    cbreak();
    init_pair(1, COLOR_GREEN, COLOR_BLACK);
    window.bkgd(COLOR_PAIR(1));
    let win_w = window.get_max_x();
    let win_h = window.get_max_y(); 

    window.addstr(&format!("Width {:?}, Height: {:?}\n\n", win_w, win_h));

    window.printw("Type things and press `delete` to quit`\n");
    window.refresh();
    window.keypad(true);
    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => { window.addch(c); },
            Some(Input::KeyDC) => break,
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => ()
        }
    }
    endwin();
}
