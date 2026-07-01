use crossterm::{execute, cursor, terminal};
use std::io::stdout;

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    execute!(out, terminal::Clear(terminal::ClearType::All))?;

    for row in 0..8 {
        for col in 0..8 {
            execute!(out, cursor::MoveTo(col * 2, row))?;
            print!(".");
        }
    }

    Ok(())
}
