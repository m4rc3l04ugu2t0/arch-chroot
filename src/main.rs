use std::io;

use crate::run_commands::run_commands;

mod run_commands;

fn main() {
    let mut input = String::new();
    println!("disk partions...");
    run_commands("lsblk", &[]);
    println!("disk path:");
    io::stdin().read_line(&mut input).expect("Invalid input");
    run_commands("fdisk", &[&input])
}
