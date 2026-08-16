use std::io;
use clearscreen;

fn menu() {
    println!("Welcome to interactive menu!");
    println!("====================================");
    
    println!("Menu:");
    println!("[1] About");
    println!("[2] Click Me!");
    println!("[0] exit");
}

fn about() {
    println!("This is my Rust exercise!");
    println!("Happy night coding :)");

    press_btn_continue::wait("Press any key to continue..").unwrap();
}

fn click_me() {
    println!("Thank you >_<");
    
    press_btn_continue::wait("Press any key to continue..").unwrap();
}

fn exit() {
    println!("Thank you for trying my program. Have a nice day ;)");

    press_btn_continue::wait("Press any key to continue..").unwrap();
}

fn main() {
    loop {
        clearscreen::clear().expect("Failed to clear screen!");
        
        menu();

        println!("Enter your choice.");

        let cmd = loop {
            let mut tmp = String::new();

            io::stdin()
                .read_line(&mut tmp)
                .expect("Can't read input!");

            let tmp = match tmp.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };

            if tmp < 0 || tmp > 2 {
                println!("Please enter a valid choice!");
                continue;
            } else {
                break tmp
            }
        };

        match cmd {
            0 => {
                clearscreen::clear().expect("Failed to clear screen!");
                exit();
                break;
            },
            1 => {
                clearscreen::clear().expect("Failed to clear screen!");
                about();
            },
            2 => {
                clearscreen::clear().expect("Failed to clear screen!");
                click_me();
            },
            _ => {
                println!("Oops :(");
            }
        }
    }
}
