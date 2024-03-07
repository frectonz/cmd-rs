use cmd_derive::{cmd_handler, Cmd};
use cmd_rs::{cmdloop, Cmd, CmdHandler, Result};

fn main() -> Result<()> {
    cmdloop(Calculator(0))?;
    Ok(())
}

/// A simple calculator app
#[derive(Cmd, Default)]
#[prompt = "Calculator >>> "]
struct Calculator(u8);

#[cmd_handler]
impl Calculator {
    /// add numbers
    fn do_add(&mut self, arg: &str) {
        let num = arg.parse::<u8>().unwrap();
        self.0 = self.0.wrapping_add(num);
    }

    /// substract numbers
    fn do_sub(&mut self, arg: &str) {
        let num = arg.parse::<u8>().unwrap();
        self.0 = self.0.wrapping_sub(num);
    }

    /// multiply numbers
    fn do_mult(&mut self, arg: &str) {
        let num = arg.parse::<u8>().unwrap();
        self.0 = self.0.wrapping_mul(num);
    }

    /// divide numbers
    fn do_div(&mut self, arg: &str) {
        let num = arg.parse::<u8>().unwrap();
        self.0 = self.0.wrapping_div(num);
    }

    fn postcmd(&self) {
        println!("Result: {}", self.0);
    }
}
