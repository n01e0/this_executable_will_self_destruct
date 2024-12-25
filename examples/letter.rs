use this_executable_will_self_destruct::ThisExecutableWillSelfDestruct;
use std::time::Duration;

fn main() {
    println!("This executable will self-destruct in five seconds");
    ThisExecutableWillSelfDestruct::new(Duration::from_secs(5)).fire();
}
