use rustyline::DefaultEditor;
pub use rustyline::Result;

pub trait Cmd {
    fn welcome(&self) -> &str;
    fn prompt(&self) -> &str;
}

pub trait CmdHandler {
    fn handler(&mut self, line: &str);
}

pub fn cmdloop(mut cmd: impl Cmd + CmdHandler) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("{}", cmd.welcome());

    loop {
        let line = rl.readline(cmd.prompt())?;
        if line == "quit" {
            break;
        }
        cmd.handler(&line);
    }

    Ok(())
}
