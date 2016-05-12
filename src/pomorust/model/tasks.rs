extern crate uuid;
use self::uuid::Uuid;
use std::thread;

pub struct Task {
    description:        String,
    uuid:               Uuid,
    pomodori_count:     u16,
    pomodori_estimate:  u16,
    comment:            String
}

impl Task {
    pub fn new(desc: &str, estimate: u16) -> Task {
        Task::new_preset(desc, Uuid::new_v4(), 0, 0, "")
    }

    pub fn new_preset(description: &str, uuid: Uuid,
           pomodori_count: u16, 
           pomodori_estimate: u16,
           comment: &str) -> Task {
        Task {
            description: description.to_string(),
            uuid: uuid,
            pomodori_count: pomodori_count,
            pomodori_estimate: pomodori_estimate,
            comment: comment.to_string()
        }
    }

    fn start(&mut self) {
        println!("Started working on : {}", self.to_string());
        wait_for(25);
        self.increment_pomodoro();
        println!("Done working on : {}", self.to_string());
    }

    fn increment_pomodoro(&mut self) {
        self.pomodori_count += 1;
    }

    pub fn to_csv(&self) -> String {
        format!("{};{};{};{};{}\n", self.description, self.uuid,
                self.pomodori_count, self.pomodori_estimate,
                self.comment)
    }

    pub fn from_csv(line: &str) -> Task {
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
}

impl ToString for Task {
    fn to_string(&self) -> String {
        format!("{}\t{}\t{} / {}", self.uuid, self.description,
                self.pomodori_count, self.pomodori_estimate)
    }
}

fn wait_for(minutes: u32) {
    thread::sleep_ms(minutes * 60 * 1000);
}
