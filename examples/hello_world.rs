use pager_rs::{CommandList, State, StatusBar};
fn main() -> std::io::Result<()> {
    let content = r#"fn main() {
    println!("Hello World!");
}"#
    .to_string();

    let status_bar = StatusBar::new("Hello World program in rust".to_string());

    let mut state = State::new(content, status_bar, CommandList::default())?;

    pager_rs::init()?;

    pager_rs::run(&mut state)?;

    pager_rs::finish()?;

    Ok(())
}
