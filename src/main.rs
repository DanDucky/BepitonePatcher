#[macro_use]
extern crate rocket;

use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::ops::Deref;
use rocket::{Build, Rocket, State};
use once_cell::sync::Lazy;
// 1.3.1
use std::sync::Mutex;
use rocket::form::validate::Contains;

static FAILED_LAYERS_EVEN: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(vec![]));
static FAILED_LAYERS_ODD: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(vec![]));

static SCAN_COUNTERS: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| Mutex::new(vec![-2, -1]));

#[get("/update/<layer>")]
fn update(layer: i32) {
    if layer != 0 {
        if layer % 2 == 0 { // even
            FAILED_LAYERS_EVEN.lock().unwrap().push(layer)
        } else {
            FAILED_LAYERS_ODD.lock().unwrap().push(layer)
        }
    }
}

#[get("/requestspecific/<requested_line>")]
fn requestspecific(requested_line : i32) -> String {
    let file = File::open(format!("static/partitions/{}", requested_line.to_string()));
    let reader = BufReader::new(file.unwrap());
    let mut lines = String::new();
    for line in reader.lines() {
        lines.push_str(&*format!("{}{}", &*line.unwrap(), "\n"));
    }
    println!("STARTING LAYER {}", requested_line);
    return lines.to_string(); // todo don't send the whole file, completely unnecessary
}

#[get("/request/<last_scan>")]
fn request(last_scan: i32) -> String {
    let mut assignment = 0;
    println!("madeit");
    if last_scan % 2 == 0 { // even
        println!("even");
        if !FAILED_LAYERS_EVEN.lock().unwrap().is_empty() {
            assignment = FAILED_LAYERS_EVEN.lock().unwrap().pop().unwrap()
        } else {
            println!("assigned new layer");
            SCAN_COUNTERS.lock().unwrap()[0] += 2;
            println!("added 2");
            assignment = SCAN_COUNTERS.lock().unwrap()[0];
            println!("finished assignment");
        }
    } else { // odd
        if !FAILED_LAYERS_ODD.lock().unwrap().is_empty() {
            assignment = FAILED_LAYERS_ODD.lock().unwrap().pop().unwrap()
        } else {
            SCAN_COUNTERS.lock().unwrap()[1] += 2;
            assignment = SCAN_COUNTERS.lock().unwrap()[1];
        }
    }
    if assignment > 800 {
        println!("complete");
        return "DISABLE\n".to_string();
    }
    println!("this is busted");
    println!("{}", assignment);
    let file = File::open(format!("static/partitions/{}", assignment.to_string()));
    println!("created file object");
    let reader = BufReader::new(file.unwrap());
    println!("created objects");
    let mut lines = String::new();
    for line in reader.lines() {
        lines.push_str(&*format!("{}{}", &*line.unwrap(), "\n"));
    }
    println!("STARTING LAYER {}", assignment);
    return lines.to_string(); // todo don't send the whole file, completely unnecessary
}
#[launch]
fn rocket() -> _ { // idk but this fixed shit
    rocket::build().mount("/", routes![request, update, requestspecific])

}