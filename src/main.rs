use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::io;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
    title: String,
    client: String,
    isDone: bool,
} 


fn create_task_list(path: PathBuf) -> Result<Vec<Task>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path);
    let mut vec_task: Vec<Task> = [].to_vec();
    for result in rdr.unwrap().deserialize() {
        let task: Task = result?;
        vec_task.push(task);
    }

    Ok(vec_task)
}

fn serialize_task_list(path: PathBuf, lst_task: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(path).unwrap();
    for task in lst_task {
        wtr.serialize(task)?;
    }
    wtr.flush()?;

    Ok(())
}

fn check_list(lst_task: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    for task in lst_task {
        println!("{:?}", task);
    }

    Ok(())
}

fn add_list(lst_task: &Vec<Task>, task_add: Task, path: PathBuf) -> Vec<Task> {
    let mut lst_task_new = lst_task.clone();
    lst_task_new.push(task_add);
    serialize_task_list(path, &lst_task_new).unwrap();


    lst_task_new
}

fn create_new_task() -> Task {
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

    Task {
        title : title.trim().to_string(), 
        client: client.trim().to_string(),
        isDone: false
    }
}


fn main() {
    let path = Path::new("test.txt");
    let display = path.display();

    let mut lst_task = create_task_list(path.to_path_buf()).unwrap();

    let mut mode = String::new();
    loop {
        mode = "".to_string();
        println!("SelectMode");
        io::stdin()
            .read_line(&mut mode)
            .expect("msg: &str");

        match mode.trim() {
            "1" => check_list(&lst_task).expect("sorry"),
            "2" => lst_task = add_list(&lst_task, create_new_task(), path.to_path_buf()),
            _   => println!("Your Input Number Is {}. This is Invalid", mode.trim())
        }
    }
}

