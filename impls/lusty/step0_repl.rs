use std::io::{self, Write};


fn read(x: String) -> String {
    x
}

fn eval(x: String) -> String {
    x
}

fn print(x: String) -> String {
    x
}

fn rep(x: String) -> String {
    print(eval(read(x)))
}

fn main()  {
    loop {
        let mut line = String::new();

        print!("user> ");
        let _ = io::stdout().flush();
        let read = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if read == 0 {
            println!();
            break;
        }
        println!("{}", rep(line));
    }
}