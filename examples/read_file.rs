use pager_rs::{run as pager_run, StatusBar};
use std::{env, fs::File, io::Read};
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let file_name = args[1].clone();

        let mut file = File::open(file_name.clone())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let status_bar = StatusBar::with_title(file_name);

        pager_run(content, status_bar)?;
    } else {
        eprintln!("Missing Filename");
    }

    Ok(())
}
