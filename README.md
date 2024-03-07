# cmd-rs

Python's cmd module implemented in Rust via proc macros. 

```rust
/// A repl that just echos
#[derive(Cmd)]
struct Echoer;

#[cmd_handler]
impl Echoer {
    /// echo message back
    fn do_echo(&mut self, arg: &str) {
        println!("Echo: {}", arg);
    }

    fn postcmd(&self) {
        println!("post command");
    }
}
```
