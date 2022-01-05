use crossterm::event::KeyCode;
use pager_rs::{run, Command, CommandList, CommandType, State, StatusBar};
fn main() -> std::io::Result<()> {
    let content = r#"Lorem ipsum dolor sit amet, consectetur adipiscing
elit. Pellentesque neque nulla, viverra ac sapien
et, ultricies convallis lectus. Suspendisse mattis
in urna quis efficitur. Quisque mollis vulputate ipsum,
ut auctor risus luctus eu. Donec sagittis convallis erat
eget imperdiet. Aliquam massa erat, venenatis eu massa at,
dignissim tempus massa. Donec blandit augue et malesuada
fermentum. In vehicula, nisl ut scelerisque sagittis,
sapien elit gravida enim, eu feugiat magna arcu sed enim.
Fusce accumsan sodales ipsum lobortis feugiat. Pellentesque
quam lectus, molestie vitae nisi a, tempor mollis mauris.
Maecenas in magna tempus, porta augue bibendum, feugiat nulla."#
        .to_string();

    let status_bar =
        StatusBar::new("Press 'p' to open selected line on seperate instance".to_string());

    let mut state = State::new(
        content,
        status_bar,
        CommandList::combine(vec![
            CommandList(vec![Command {
                cmd: vec![CommandType::Key(KeyCode::Char('p'))],
                desc: "Open selected line on seperate instance".to_string(),
                func: &|state| {
                    let commands =
                        CommandList::combine(vec![CommandList::quit(), CommandList::navigation()]);

                    let mut modal = State::new(
                        state.content.lines().nth(state.pos.1).unwrap().to_string(),
                        StatusBar::new("Quit (q)".to_string()),
                        commands,
                    )
                    .unwrap();
                    modal.show_line_numbers = false;
                    run(&mut modal).unwrap();
                    true
                },
            }]),
            CommandList::quit(),
            CommandList::navigation(),
            CommandList::help(),
        ]),
    )?;
    state.show_line_numbers = false;

    pager_rs::init()?;

    pager_rs::run(&mut state)?;

    pager_rs::finish()?;

    Ok(())
}
