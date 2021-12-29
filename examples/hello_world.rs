use pager_rs::StatusBar;
fn main() -> std::io::Result<()> {
    let content = r#"fn main() {
    println!("Hello World!");
}"#
    .to_string();

    let status_bar = StatusBar::with_title("Hello World program in rust".to_string());

    pager_rs::run(content, status_bar)?;

    Ok(())
}
