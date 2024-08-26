use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use colored::Colorize;

fn main() {
    // let loading_chars: [char; 2] = ['▫', '▪'];
    // let loading_chars: [char; 4] = ['░', '▒', '▓', '█'];
    // let loading_chars: [char; 4] = ['⋮', '⋰', '⋯', '⋱'];
    let loading_chars: [char; 6] = ['⠄', '⠤', '⠦', '⠶', '⠷', '⠿'];
    let full_str = "⠿";
    let empty_str = " ";

    let interval_millis = 50;
    let total_task_count = 300;
    let mut task_count = 0;

    loop {
        let fraction = task_count as f64 / loading_chars.len() as f64;
        let total_blocks = (total_task_count as f64 / loading_chars.len() as f64).ceil() as usize;

        let full = full_str.repeat(fraction.floor() as usize);

        let remainder = task_count as f64 % loading_chars.len() as f64;

        let one_less = total_task_count - 1;
        let loading = match task_count == one_less {
            true => full_str.to_string().green(),
            false => format!("{}", loading_chars[(remainder - 1.0).floor() as usize]).red(),
        };

        let empty = empty_str.repeat(total_blocks - fraction.floor() as usize - 1);

        let load_string = format!("{}{}{}", full.green(), loading, empty);
        // println!("|{load_string}|");
        print!("\r[{load_string}]");
        let _ = io::stdout().flush();

        task_count += 1;

        if task_count == total_task_count {
            break;
        }

        thread::sleep(Duration::from_millis(interval_millis));
    }
}
