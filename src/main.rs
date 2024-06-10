mod options;

use a_rusty_cli::{Command, CommandError, CommandManager, Opt, OptValue};

struct EchoCommand;

impl Command for EchoCommand {
    fn run(
        &self,
        args: Vec<String>,
        opts: std::collections::HashMap<Opt, OptValue>,
    ) -> Result<(), CommandError> {
        let _ = opts;
        let output = args.join(" ");
        println!("{}", output);
        Ok(())
    }
}

struct LsCommand;

impl Command for LsCommand {
    /// Returns the options available for the `ls` command.
    fn opts(&self) -> Vec<Opt> {
        vec![
            Opt::new(
                "a",
                "all",
                OptValue::BOOL(false),
                "do not ignore entries starting with .",
            ),
            Opt::new(
                "l",
                "long",
                OptValue::BOOL(false),
                "use a long listing format",
            ),
            Opt::new(
                "h",
                "human-readable",
                OptValue::BOOL(false),
                "with -l and -s, print sizes like 1K 234M 2G etc.",
            ),
        ]
    }

    /// Executes the `ls` command with the given arguments and options.
    fn run(
        &self,
        args: Vec<String>,
        opts: std::collections::HashMap<Opt, OptValue>,
    ) -> Result<(), CommandError> {
        println!(
            "Running ls command with args: {:?} and opts {:?}",
            args, opts
        );

        // Do ls command logic here

        let mut all = false;

        if let Some(opt) = a_rusty_cli::find_opt(opts, "--all".to_string()) {
            if let OptValue::BOOL(value) = opt {
                all = value;
            }
        }
        println!("all: {}", all);

        for entry in std::fs::read_dir(".").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let file_name = path.file_name().unwrap().to_str().unwrap().to_owned();

            if !all && file_name.starts_with(".") {
                continue;
            }

            println!("{}", file_name);
        }

        Ok(())
    }
}

fn main() {
    let mut manager = CommandManager::new();

    manager.add_command("echo".to_string(), Box::new(EchoCommand));
    manager.add_command("ls".to_string(), Box::new(LsCommand));
    manager.add_command("test".to_string(), Box::new(options::TestCommand));

    let _ = manager.run(std::env::args().skip(1).collect());
}
