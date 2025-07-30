use chrono::prelude::*;


fn main() {
    let local: DateTime<Local> = Local::now();
    println!("The time is now! {}",local.to_rfc2822());
}