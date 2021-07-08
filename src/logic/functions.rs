use std::io::Write;
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
    style::Stylize
};

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

/// Draws  a box with an colored center, this center can be taken as a reference point after running the given cursor command.
pub fn draw_cursor_box<W, F, T>(w: &mut W, description: &str, cursor_command: F) -> Result<()>
where
    W: Write,
    F: Fn(u16, u16) -> T,
    T: Command,
{
    execute!(
        w,
        cursor::Hide,
        cursor::MoveTo(0, 0),
        style::SetForegroundColor(style::Color::Red),
        style::Print(format!(
            "Red box is the center. After the action: '{}' another box is drawn.",
            description
        ))
    )?;

    let start_y = 2;
    let width = 21;
    let height = 11 + start_y;
    let center_x = width / 2;
    let center_y = (height + start_y) / 2;

    for row in start_y..=10 + start_y {
        for column in 0..=width {
            if (row == start_y || row == height - 1) || (column == 0 || column == width) {
                queue!(
                    w,
                    cursor::MoveTo(column, row),
                    style::PrintStyledContent("▓".red())
                )?;
            } else {
                queue!(
                    w,
                    cursor::MoveTo(column, row),
                    style::PrintStyledContent("_".red().on_white())
                )?;
            }
        }
    }

    queue!(
        w,
        cursor::MoveTo(center_x, center_y),
        style::PrintStyledContent("▀".red().on_white())
    )?;
    queue!(
        w,
        cursor_command(center_x, center_y),
        style::PrintStyledContent("√".magenta().on_white())
    )?;
    w.flush()?;
    Ok(())
}