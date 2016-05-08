#![feature(type_ascription)]
extern crate uuid;
extern crate argparse;

pub mod pomorust;

use pomorust::config::config::read_task_file;
use pomorust::actions;

fn main() {
    let task_file = read_task_file();
    actions::parse();
}
