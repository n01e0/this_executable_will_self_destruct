use std::{env, thread, time::Duration, process::Command};

pub struct ThisExecutableWillSelfDestruct
{
    seconds: Duration,
}

impl ThisExecutableWillSelfDestruct 
{
    pub fn new(seconds: Duration) -> Self
    {
        Self {
            seconds,
        }
    }

    pub fn fire(self) {
        let ThisExecutableWillSelfDestruct { seconds } = self;
        thread::sleep(seconds);

        let delete_handle = thread::spawn(move || {
            let exe = env::current_exe().expect("Failed to get current executable");

            if cfg!(target_os = "windows") {
                let mut cmd = Command::new("cmd");
                cmd.args(&["/C", "del", exe.to_str().unwrap()]);
                if let Err(e) = cmd.spawn() {
                    eprintln!("Failed to spawn delete process: {}", e);
                }
            } else {
                let mut cmd = Command::new("sh");
                cmd.args(&["-c", &format!("rm {}", exe.to_str().unwrap())]);
                if let Err(e) = cmd.spawn() {
                    eprintln!("Failed to spawn delete process: {}", e);
                }
            }
        });

        let _ = delete_handle.join();
    }
}
