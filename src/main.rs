#![allow(dead_code)]

use crate::trash::trash;
use std::{io::stdin, error::Error};

mod deck;
mod trash;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("[T]: Trash");
    // let mut stin_read_buffer = String::new();
    // stdin().read_line(&mut stin_read_buffer)?;
    // let lowercase: String = stin_read_buffer.to_ascii_lowercase();
    // let input: &str = lowercase.trim();
    // match input {
    //     "t" => trash(),
    //     _ => Ok(())
    // }
    trash()
}
