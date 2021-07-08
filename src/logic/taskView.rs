use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::io;
use std::process;
use std::fmt;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, 
    style::Stylize
};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::functions;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    title: String,
    client: String,
    date  : String,
    isDone: bool,
} 

impl Task {
    fn get_title_length(&self) -> u32 {
        self.title.chars().fold(0, |x, y: char| if y.len_utf8() > 1 {x+2} else {x+1})
    }

    fn get_client_length(&self) -> u32 {
        self.client.chars().fold(0, |x, y: char| if y.len_utf8() > 1 {x+2} else {x+1})
    }

    fn get_date_length(&self) -> u32 {
        self.date.chars().fold(0, |x, y: char| if y.len_utf8() > 1 {x+2} else {x+1})
    }

    fn get_filled_space(&self, x: u32, str_now: &String) -> String {
        let mut space = String::new();
        for i in 1..x+1 {
            space = space + " ";
        }
        str_now.clone() + &space
    }
}


impl fmt::Display for Task{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let title_length = 20;
        let client_length = 8;
        let date_length = 30;

        match self.isDone {
            true  => write!(f, "☑ {} | {} | {}", self.get_filled_space(title_length - self.get_title_length(), &self.title)
                                                         , self.get_filled_space(client_length - self.get_client_length(), &self.client)
                                                         , self.get_filled_space(date_length - self.get_date_length(), &self.date)),

            false => write!(f, "  {} | {} | {}", self.get_filled_space(title_length - self.get_title_length(), &self.title)
                                                         , self.get_filled_space(client_length - self.get_client_length(), &self.client)
                                                         , self.get_filled_space(date_length - self.get_date_length(), &self.date))
        }
    }
}

pub fn create_task_list(path: PathBuf) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path);
    let mut vec_task: Vec<Task> = [].to_vec();
    for result in rdr.unwrap().deserialize() {
        let task: Task = result.unwrap();
        vec_task.push(task);
    }

    Ok(vec_task)
}

fn serialize_task_list(path: PathBuf, lst_task: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path).unwrap();
    for task in lst_task {
        wtr.serialize(task);
    }
    wtr.flush();

    Ok(())
}

fn check_list(lst_task: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    for task in lst_task {
        println!("{}", task);
    }

    Ok(())
}

fn add_list(lst_task: &Vec<Task>, task_add: Task, path: PathBuf) -> Vec<Task> {
    let mut lst_task_new = lst_task.clone();
    lst_task_new.push(task_add);
    serialize_task_list(path, &lst_task_new).unwrap();


    lst_task_new
}

fn create_new_task() -> Result<Task, Box<dyn Error>> {
    let mut title = String::new();
    println!("input title...");
    io::stdin()
        .read_line(&mut title)
        .expect("msg: &str");

    let mut client = String::new();
    println!("input client...");
    io::stdin()
        .read_line(&mut client)
        .expect("msg: &str");

    let mut date = String::new();
    println!("input date...");
    io::stdin()
        .read_line(&mut date)
        .expect("msg: &str");

    let date_year: i32 = date[0..2].to_string().parse().unwrap();
    let date_month: u32 = date[2..4].to_string().parse().unwrap();
    let date_day: u32 = date[4..6].to_string().parse().unwrap();

    let date_chrono = Utc.ymd(2000 + date_year, date_month, date_day);
    println!("{:?}", date_chrono);

    Ok(Task {
        title : title.trim().to_string(),
        client: client.trim().to_string(),
        date  : date_chrono.to_string(),
        isDone: false
    })
}

pub fn run<W>(w: &mut W) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    let path = Path::new("test.txt");
    let display = path.display();

    let mut lst_task = create_task_list(path.to_path_buf()).unwrap();

    execute!(w, terminal::EnterAlternateScreen);

    //terminal::enable_raw_mode()?;

    let mut bIsFirst = true;

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            style::PrintStyledContent("████████████".red()),
            cursor::MoveTo(0,0),
        );

        for task in &lst_task{
            queue!(w,
                   style::Print(task), 
                   cursor::MoveToNextLine(1),
            );
        }

        // 初回だけflushする
        if bIsFirst { w.flush(); bIsFirst = false; }

        queue!(
            w,
            cursor::RestorePosition,
            cursor::MoveLeft(2)
        );


        match functions::read_char().unwrap() {
            'j' => functions::draw_cursor_box(w, "Move Left (2)", |_, _| cursor::MoveDown(1))?,
            'k' => functions::draw_cursor_box(w, "Move Left (2)", |_, _| cursor::MoveUp(1))?,
            'c' => lst_task[cursor::position().unwrap().1 as usize].isDone = !lst_task[cursor::position().unwrap().1 as usize].isDone,
            'q' => break,
            _ => {},
        };
    }


    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    );

    terminal::disable_raw_mode();

    serialize_task_list(path.to_path_buf(), &lst_task);

    Ok(())
}