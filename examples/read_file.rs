use pager_rs::{CommandList, State, StatusBar};
use std::{env, fs::File, io::Read};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let file_name = args[1].clone();

        let mut file = File::open(file_name.clone())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let status_bar = StatusBar::new(file_name);

        let mut state = State::new(content, status_bar, CommandList::default())?;

        pager_rs::init()?;

        pager_rs::run(&mut state)?;

        pager_rs::finish()?;
    } else {
        eprintln!("Missing Filename");
    }

    Ok(())
}
