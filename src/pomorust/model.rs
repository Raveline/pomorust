use uuid::Uuid;
use chrono;
use pomorust::utils::{wait_for, MaybeLocalDate, parse_maybe_local_date};


#[derive(Debug)]
pub struct Task {
    /// Main description for the task
    pub description: String,
    /// Single generated identifier for the task
    pub uuid: Uuid,
    /// Number of pomodori spent on this task
    pomodori_count: u16,
    /// Number of pomodori needed according to user
    pomodori_estimate: u16,
    /// Is the user currently doing a pomodoro over this task ?
    pub is_ongoing: bool,
    /// Comment
    comment: String,
    /// When (and if) this task was started for the first time
    start_date: MaybeLocalDate,
    /// When (and if) this task was finished
    end_date: MaybeLocalDate
}

impl Task {
    pub fn new(desc: &str, estimate: u16) -> Task {
        Task {
            description: desc.to_string(),
            uuid: Uuid::new_v4(),
            pomodori_count: 0,
            pomodori_estimate: estimate,
            comment: "".to_string(),
            is_ongoing: false,
            start_date: None,
            end_date: None
        }
    }

    pub fn start(&mut self) {
        if self.start_date.is_none() {
            self.start_date = Some(chrono::Local::now())
        }
        self.is_ongoing = true;
    }

    pub fn do_one_pomodoro(&mut self) {
        wait_for(25);
        self.increment_pomodoro();
        self.is_ongoing = false;
    }

    pub fn finish(&mut self) {
        self.end_date = Some(chrono::Local::now())
    }

    fn increment_pomodoro(&mut self) {
        self.pomodori_count += 1;
    }

    pub fn can_be_identified_by(&self, identifier: &str) -> bool {
        self.uuid.to_string().starts_with(identifier)
    }

    pub fn is_finished(&self) -> bool {
        self.end_date.is_some()
    }

    pub fn to_csv(&self) -> String {
        let start_date_string = self.start_date.map_or(String::new(), |x|x.to_rfc3339());
        let end_date_string = self.end_date.map_or(String::new(), |x| x.to_rfc3339());
        format!("{};{};{};{};{};{};{};{}\n", self.description, self.uuid,
                self.pomodori_count, self.pomodori_estimate,
                self.comment, self.is_ongoing,
                start_date_string,
                end_date_string
        )
    }

    pub fn from_csv(line: &str) -> Task {
        let task_elements = line.split(";").collect::<Vec<&str>>();
        let desc = task_elements[0];
        let uuid : Uuid = Uuid::parse_str(task_elements[1])
            .ok()
            .expect("Error in the task file : uuid not parsable.");
        let pomodori_count: u16 = task_elements[2].parse()
            .ok()
            .expect("Error in the task file : pomodori count not parsable.");
        let pomodori_estimate: u16 = task_elements[3].parse()
            .ok()
            .expect("Error in the task file : pomodori count not parsable");
        let comment = task_elements[4];
        let is_ongoing : bool = task_elements[5] == "true";
        let start_date = parse_maybe_local_date(task_elements[6],
            "Error in the task file : start date not parsable.");
        let end_date = parse_maybe_local_date(task_elements[7],
            "Error in the task file : start date not parsable.");
        Task {
            description: desc.to_string(),
            uuid: uuid,
            pomodori_count: pomodori_count,
            pomodori_estimate: pomodori_estimate,
            comment: comment.to_string(),
            is_ongoing: is_ongoing,
            start_date: start_date,
            end_date: end_date
        }
    }

    pub fn to_list_line(&self) -> String {
        let ongoing_sign = match self.is_ongoing {
            false => "-",
            true => "!"
        };
        format!("{0} {1: <38} {2:.<80} {3} / {4}",
                ongoing_sign, self.uuid.to_string(), self.description,
                self.pomodori_count, self.pomodori_estimate)
    }
}

impl ToString for Task {
    fn to_string(&self) -> String {
        format!("{}", self.description)
    }
}


pub struct Context {
    /// Should OS level notification be used
    pub use_notification: bool,
    /// Should sounds be played
    pub use_sound: bool,
    /// Listing of tasks recorded by the user
    pub tasks: Vec<Task>,
    /// When was the last pomodoro done ?
    pub last_pomodoro: MaybeLocalDate,
    /// How many sequential pomodori were run ?
    pub pomodori_count: u16
}

#[derive(Debug)]
pub enum IdentificationError {
    NoMatch,
    AmbiguousMatch
}

impl Context {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn has_ongoing_task(&self) -> bool {
        let ongoings = self.tasks.iter().find(|&x| x.is_ongoing);
        ongoings.is_some()
    }

    /// Check if a given identifier identify one and only one
    /// task.
    pub fn is_valid_identifier(&self, identifier: &str) -> Result<(), IdentificationError> {
        let mut matching = self.tasks
            .iter()
            .filter(|&x| x.can_be_identified_by(&identifier));
        match matching.next() {
            None => Err(IdentificationError::NoMatch),
            Some(_) => match matching.next() {
                None => Ok(()),
                Some(_) => Err(IdentificationError::AmbiguousMatch)
            }
        }
    }

    pub fn get_task(&mut self, identifier: &str) -> &mut Task {
        self.tasks
            .iter_mut()
            .find(|x| x.can_be_identified_by(&identifier))
            .unwrap()
    }

    pub fn get_current_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|&x| !x.is_finished()).collect::<Vec<&Task>>()
    }

    /// Context must manage a basic idea of the pomodoro technique: regular pauses.
    /// After each pomodori, one should take a 5 minute pauses.
    /// Every four pomodori, a longer pause should be taken.
    /// However,pomorust demands that the user regularly inputs on what task he's
    /// working, and the count of successive pomoodori could easily be wrong.
    /// So we will only count pomodori as successive if the last pomodoro count
    /// was less than 40 minutes ago.
    pub fn last_pomodoro_was_recent(&self) -> bool {
        let right_now = chrono::Local::now();
        match self.last_pomodoro {
            None => false,
            Some(t) => (right_now - t).num_minutes() <= 40
        }
    }

    pub fn increment_pomodoro_count(&mut self) {
        if self.last_pomodoro_was_recent() {
            self.pomodori_count += 1 ;
            if self.pomodori_count > 3 {
                self.pomodori_count = 0;
            }
        } else {
            self.pomodori_count = 1;
        }
        self.last_pomodoro = Some(chrono::Local::now());
    }

    pub fn should_be_long_pause(&self) -> bool {
        self.last_pomodoro_was_recent() && self.pomodori_count == 3
    }
}
