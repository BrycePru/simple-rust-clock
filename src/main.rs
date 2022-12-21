use chrono::{Local}; ///use chrono crate for local time

// https://www.w3.org/TR/xml-entity-names/025.html
const DIGITS : [[&str; 11]; 7] = [
    ["┏━┓ ","  ╻  "," ┏━┓ ", " ┏━┓ "," ╻ ╻ "," ┏━┓ "," ┏   "," ┏━┓ "," ┏━┓ "," ┏━┓ ","   "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ "," ╻ "],
    ["┃ ┃ ","  ┃  ","   ┃ ", "   ┃ "," ┃ ┃ "," ┃   "," ┃   ","   ┃ "," ┃ ┃ "," ┃ ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┏━┛ ", " ┣━┫ "," ┗━┫ "," ┗━┓ "," ┣━┓ ","   ┃ "," ┣━┫ "," ┗━┫ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ ","   "],
    ["┃ ┃ ","  ┃  "," ┃   ", "   ┃ ","   ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ┃ ┃ ","   ┃ "," ╹ "],
    ["┗━┛ ","  ╹  "," ┗━━ ", " ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   ╹ "," ┗━┛ "," ┗━┛ ","   "],
]; ///initializes the display of each digit



fn main() {
    print!("\x1b[2J"); ///Erases the display
    print!("\x1b[?25l"); ///Hides the cursor
    loop {
        let t = Local::now(); ///local time
        let time = t.format("%H:%M:%S").to_string();///formating the time as Hour:Minute:Second
        ///iterates over the rows of the DIGITS variable
        for row in &DIGITS {
            ///iterating over every character of the time string
            for c in time.chars() {
                let col = match c {
                    ///reading the characters into displaying the number by matching from the string
                    '0'..='9' => c as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                ///printing the resulting column from the row
                print!("{} ", row[col]);
            }
            println!(); ///prints an empty line
        }
        std::thread::sleep(std::time::Duration::from_millis(999)); ///prints time after 999 miliseconds
        print!("\x1b[7A"); /// moves the character cursor up seven times
    }
}
