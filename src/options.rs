use a_rusty_cli::{find_opt, Command, CommandError, Opt, OptValue};

pub struct TestCommand;

impl Command for TestCommand {
    fn opts(&self) -> Vec<Opt> {
        vec![
            Opt::new(
                "a",
                "all",
                OptValue::STRING("".to_owned()),
                "do not ignore entries starting with .",
            ),
            Opt::new("l", "long", OptValue::INT64(0), "use a long listing format"),
            Opt::new(
                "h",
                "human-readable",
                OptValue::UINT64(0),
                "with -l and -s, print sizes like 1K 234M 2G etc.",
            ),
        ]
    }

    fn run(
        &self,
        args: Vec<String>,
        opts: std::collections::HashMap<Opt, OptValue>,
    ) -> Result<(), CommandError> {
        let output = args.join(" ");
        println!("{}", output);

        let mut all = "".to_owned();
        let mut long = 0;
        let mut human_readable: u64 = 0;

        if let Some(opt) = find_opt(opts.clone(), "--all".to_owned()) {
            match opt {
                OptValue::STRING(value) => {
                    all = value;
                }
                _ => {}
            }
        }

        if let Some(opt) = find_opt(opts.clone(), "--long".to_owned()) {
            match opt {
                OptValue::INT64(value) => {
                    long = value;
                }
                _ => {}
            }
        }

        if let Some(opt) = find_opt(opts.clone(), "--human-readable".to_owned()) {
            match opt {
                OptValue::UINT64(value) => {
                    human_readable = value;
                }
                _ => {}
            }
        }

        println!("all: {:?}", all);
        println!("long: {:?}", long);
        println!("human_readable: {:?}", human_readable);

        Ok(())
    }
}
