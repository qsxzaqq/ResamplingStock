use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();

    loop {
        io::stdin().read_line(&mut buffer).unwrap();
    
        let datetime = &buffer[0..19];
        let mode = &buffer[20..];
    
        if mode.starts_with("5min") {
            let mut hour = datetime[11..13].parse::<i32>().unwrap();
            let mut minute = datetime[14..16].parse::<i32>().unwrap();
            let mut second = datetime[17..19].parse::<i32>().unwrap();
    
            if minute == 0 {
                second = 0;
            } else if minute > 0 && minute <= 5 {
                minute = 5;
                second = 0;
            } else if minute > 5 && minute <= 10 {
                minute = 10;
                second = 0;
            } else if minute > 10 && minute <= 15 {
                minute = 15;
                second = 0;
            } else if minute > 15 && minute <= 20 {
                minute = 20;
                second = 0;
            } else if minute > 20 && minute <= 25 {
                minute = 25;
                second = 0;
            } else if minute > 25 && minute <= 30 {
                minute = 30;
                second = 0;
            } else if minute > 30 && minute <= 35 {
                minute = 35;
                second = 0;
            } else if minute > 35 && minute <= 40 {
                minute = 40;
                second = 0;
            } else if minute > 40 && minute <= 45 {
                minute = 45;
                second = 0;
            } else if minute > 45 && minute <= 50 {
                minute = 50;
                second = 0;
            } else if minute > 50 && minute <= 55 {
                minute = 55;
                second = 0;
            } else if minute > 55 {
                hour += 1;
                minute = 0;
                second = 0;
            }
    
            io::stdout().write(format!("{}{:02}:{:02}:{:02}\n", &datetime[0..11], hour, minute, second).as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            buffer.clear();
        } else if mode.starts_with("15min") {
            let mut hour = datetime[11..13].parse::<i32>().unwrap();
            let mut minute = datetime[14..16].parse::<i32>().unwrap();
            let mut second = datetime[17..19].parse::<i32>().unwrap();
    
            if minute == 0 {
                second = 0;
            } else if minute > 0 && minute <= 15 {
                minute = 15;
                second = 0;
            } else if minute > 15 && minute <= 30 {
                minute = 30;
                second = 0;
            } else if minute > 30 && minute <= 45 {
                minute = 45;
                second = 0;
            } else if minute > 45 {
                hour += 1;
                minute = 0;
                second = 0;
            }
    
            io::stdout().write(format!("{}{:02}:{:02}:{:02}\n", &datetime[0..11], hour, minute, second).as_bytes()).unwrap();
            io::stdout().flush().unwrap();
            buffer.clear();
        }
    }
}