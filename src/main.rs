#![allow(clippy::cognitive_complexity)]

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::env;
use task_manager::logic::taskView;
use task_manager::logic::addTaskView;
use task_manager::logic::functions;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

const MENU: &str = r#"TaskManager Ver1.0.0.0

Controls:

 - 'q' - quit interactive test (or return to this menu)
 - any other key - continue with next step

Available tests:

1. View TaskLists
2. AddTasks
3. None
4. None

Select command to run ('1', '2', ...) or hit 'q' to quit.
"#;

fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let path = Path::new("test.txt");

    let list = 

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Show,
            cursor::MoveTo(0, 0)
        )?;

        for line in MENU.split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        match functions::read_char().unwrap() {
            '1' => taskView::run(w).unwrap(),
            '2' => addTaskView::run(w).unwrap(),
            'q' => break,
            _ => {}
        };
    };

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

fn check_pass<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    //terminal::enable_raw_mode()?;

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Show,
            cursor::MoveTo(1, 1)
        )?;

        execute!(w,
            style::Print("Input Your PassCode."),
            cursor::MoveToNextLine(1),
        )?;

        let mut pass = String::new();
        io::stdin()
            .read_line(&mut pass)
            .expect("msg: &str");

        if pass.trim() == "pass" {
            break;
        }

        w.flush()?;
    }


    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn buffer_size() -> Result<(u16, u16)> {
    terminal::size()
}

fn main() -> Result<()> {
    env::set_current_dir(env::current_exe().unwrap().parent().unwrap())?;

    let mut stdout = io::stdout();
    check_pass(&mut stdout)?;
    run(&mut stdout)
}
