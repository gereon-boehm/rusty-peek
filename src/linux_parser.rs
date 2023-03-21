use ncurses::*;
use super::processes::Process;
use super::processes::Format;

pub struct System;

pub struct NCursesDisplay;

impl System {
    // TODO: Return a container composed of the system's processes
    fn processes(&self) -> Vec<Process> { return Vec::new();}
}

impl NCursesDisplay {

    fn display_processes(processes: &Vec<Process>, window: WINDOW, n: i32) {
        let row = 1;
        let pid_column = 2;
        let user_column = 9;
        let cpu_column = 16;
        let ram_column = 26;
        let time_column = 35;
        let command_column = 46;
        let color_pair = COLOR_PAIR(2);
        attron(color_pair);
        mvwprintw(window, row, pid_column, "PID");
        mvwprintw(window, row, user_column, "USER");
        mvwprintw(window, row, cpu_column, "CPU[%%]");
        mvwprintw(window, row, ram_column, "RAM[MB]");
        mvwprintw(window, row, time_column, "TIME+");
        mvwprintw(window, row, command_column, "COMMAND");
        attroff(color_pair);
        let num_processes = std::cmp::min(processes.len().try_into().unwrap(), n);
        for i in 0..num_processes {
            let idx = i as usize;
            mvwprintw(window, row + 1 + i, pid_column, processes[idx].pid().to_string().as_str());
            mvwprintw(window, row + 1 + i, user_column, processes[idx].user().as_str());
            let cpu = processes[idx].cpu_utilization() * 100.0;
            mvwprintw(window, row + 1 + i, cpu_column, format!("{:.2}", cpu).as_str());
            mvwprintw(window, row + 1 + i, ram_column, processes[idx].ram().as_str());
            let up_time = Format::elapsed_time(processes[idx].up_time());
            mvwprintw(window, row + 1 + i, time_column, up_time.as_str());
            mvwprintw(window, row + 1 + i, command_column, &processes[idx].command()[..(getmaxx(window) as usize - command_column as usize)]);
        }
    }

    #[allow(unreachable_code)]
    pub fn display(system: System, n: i32) {
        initscr();            // start ncurses
        noecho();             // do not print input values
        cbreak();             // terminate ncurses on ctrl + c
        start_color();        // enable color

        let x_max = getmaxx(stdscr());
        let system_window = newwin(9, x_max - 1, 0, 0);
        let y_max = getmaxy(system_window);
        let process_window = newwin(3 + n, x_max - 1, y_max + 1, 0);

        loop {
            init_pair(1, COLOR_BLUE, COLOR_BLACK);
            init_pair(2, COLOR_GREEN, COLOR_BLACK);
            box_(system_window, 0, 0);
            box_(process_window, 0, 0);
            Self::display_processes(&system.processes(), process_window, n);
            wrefresh(system_window);
            wrefresh(process_window);
            refresh();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        endwin();
    }

}