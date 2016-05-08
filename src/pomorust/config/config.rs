use std::fs::File;
use std::io::Read;
use std::string::ToString;
use pomorust::model::tasks::Task;
use std::option;
use uuid;

pub fn read_task_file() -> Option<Vec<Task>> {
    let mut file = match File::open("tasks") {
        Ok(file) => file,
        Err(_) => return None
    };
    let mut file_txt = String::new();
    match file.read_to_string(&mut file_txt) {
        Ok(_) => {},
        Err(_) => return None
    };
    let mut data: Vec<Task> = Vec::new();
    for l in file_txt.split("\n").collect::<Vec<&str>>() {
        if l.len() > 0 {
            data.push(read_task_line(l));
        }
    }
    Some(data)
}

fn read_task_line(line: &str) -> Task {
    let task_elements = line.split(";").collect::<Vec<&str>>();
    let desc = task_elements[0];
    let uuid : uuid::Uuid = uuid::Uuid::parse_str(task_elements[1])
        .ok()
        .expect("Error in the task file : uuid not parsable.");
    let pomodori_count: u16 = task_elements[2].parse()
        .ok()
        .expect("Error in the task file : pomodori count not parsable.");
    let pomodori_estimate: u16 = task_elements[3].parse()
        .ok()
        .expect("Error in the task file : pomodori count not parsable");
    let comment = task_elements[4];
    Task::new_preset(desc, uuid, pomodori_count, pomodori_estimate, comment)
}
