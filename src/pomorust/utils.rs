use std::env;
use std::thread;
use std::time::Duration;
use std::process;
use std::os::unix::process::CommandExt;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

pub fn wait_for(minutes: u64) {
    thread::sleep(Duration::new(minutes * 60, 0));
}

pub fn notify(title: &str, text: &str) {
    Notification::new().summary(title)
        .body(text)
        .hint(Hint::SuppressSound(true))
        .show().unwrap();
}

/// We want to leave the shell available once the pomodoro has started.
/// So we need to launch the timer as an independant process.
/// This function is the shortcut to do this.
/// Rust process library is relatively unstable at this point,
/// so we want to encapsulate this.
pub fn run_background_process(task_id: String) {
	process::Command::new(env::args().nth(0).expect("Should not happen"))
		.arg("new_pomodoro")
        .arg(task_id)
        .before_exec(|| { Ok(()) }).spawn().unwrap();
		//.session_leader(true).spawn().unwrap();
}
