use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() {
    // let loading_chars: [char; 2] = ['▫', '▪'];
    // let loading_chars: [char; 4] = ['░', '▒', '▓', '█'];
    let loading_chars: [char; 4] = ['⋮', '⋰', '⋯', '⋱'];

    println!("Loader Test");

    let mut i = 0;
    loop {
        let l = loading_chars[i];
        print!("\r{l} Loading...");
        let _ = io::stdout().flush();

        if i == loading_chars.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
        thread::sleep(Duration::from_millis(120));
    }
}
