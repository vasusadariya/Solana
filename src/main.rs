use chrono::{Local,Utc};

fn main(){
    let local= Local::now();
    println!("Local time is {}",local);
    let utc= Utc::now();
    println!("Utc time is {}",utc);
}