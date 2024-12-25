# This executable will self destruct

## usage

```rust
use this_executable_will_self_destruct::ThisExecutableWillSelfDestruct;
use std::time::Duration;

fn main() {
    println!("This executable will self-destruct in five seconds");
    ThisExecutableWillSelfDestruct::new(Duration::from_secs(5)).fire();
}
```

```sh
$ cargo build --examples
$ ./target/debug/examples/letter
This executable will self-destruct in five seconds
$ stat ./target/debug/examples/letter
stat: cannot statx './target/debug/examples/letter': No such file or directory
```

