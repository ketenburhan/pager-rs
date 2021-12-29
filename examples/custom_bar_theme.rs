use crossterm::style::{Attribute, Color, ContentStyle, Stylize};
use pager_rs::StatusBar;
fn main() -> std::io::Result<()> {
    let content = r#"fn main() {
    println!("Hello World!");
}"#
    .to_string();

    let theme = ContentStyle::new()
        .with(Color::White)
        .on(Color::Red)
        .attribute(Attribute::Italic);
    let status_bar = StatusBar::with_theme(
        "Hello World program in rust with colored status bar".to_string(),
        theme,
    );

    pager_rs::run(content, status_bar)?;

    Ok(())
}
