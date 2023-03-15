use ncurses::*;

pub struct NCursesDisplay {}

impl NCursesDisplay {

    #[allow(unreachable_code)]
    pub fn display(n: i32) {
        initscr();            // start ncurses
        noecho();             // do not print input values
        cbreak();             // terminate ncurses on ctrl + c
        start_color();        // enable color

        let x_max = getmaxx(stdscr());
        let system_window = newwin(9, x_max - 1, 0, 0);
        let process_window = newwin(3 + n, x_max - 1, getmaxy(system_window) + 1, 0);

        loop {
            init_pair(1, COLOR_BLUE, COLOR_BLACK);
            init_pair(2, COLOR_GREEN, COLOR_BLACK);
            box_(system_window, 0, 0);
            box_(process_window, 0, 0);
            wrefresh(system_window);
            wrefresh(process_window);
            refresh();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        endwin();
    }

}