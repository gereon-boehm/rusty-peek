mod linux_parser;
use crate::linux_parser::NCursesDisplay;
use linux_parser::System;

fn main() {
    NCursesDisplay::display(System {}, 10);
}
