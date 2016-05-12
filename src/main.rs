#![feature(type_ascription)]
extern crate uuid;
extern crate argparse;

pub mod pomorust;

use pomorust::config::config;
use pomorust::actions;

fn main() {
    let task_file = config::create_context();
    actions::parse();
}
