// use std::env;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
// use std::fs;

// fn main() {
//     let url = "https://www.rust-lang.org/";
//     let output = "rust-lang";
//     println!("Fetching url {}", url);
//     let body = reqwest::blocking::get(url).unwrap().text().unwrap();

//     println!("Convering HTML To Markdown");

//     let markdown = html2md::parse_html(&body);

//     fs::write(output, markdown.as_bytes()).unwrap();
//     println!("Done");
//     println!("Convering markdown has been save in {}", output);
// }

// fn pi() -> f64 {
//     3.1415926
// }
// fn not_pi() {
//     3.1415926;
// }

// fn main() {
//     let pi = pi();
//     let is_unit1 = not_pi();
//     println!("is_pi: {:?}, is_unit1: {:?}", pi, is_unit1);
//     let args: Vec<String> = env::args().collect();
//     println!("env: {:?}", args);
//     assert_eq(2 + 2, 4);
// }

// fn assert_eq(a: i32, b: i32) -> i32 {
//     if a == b {
//         return a;
//     } else {
//         panic!("assertion failed: a != b");
//     };
// }

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The Secret number is {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("your guess: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
