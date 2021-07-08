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
    //execute!(
        //w,
        //cursor::Hide,
        //cursor::MoveTo(0, 0),
        //style::SetForegroundColor(style::Color::Red),
        //style::Print(format!(
            //"Red box is the center. After the action: '{}' another box is drawn.",
            //description
        //))
    //)?;

    queue!(
        w,
        cursor_command(0, 0),
        style::PrintStyledContent("▓".red()),
        cursor::SavePosition
    )?;

    w.flush()?;
    Ok(())
}