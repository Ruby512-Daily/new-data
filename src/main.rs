use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {}.", secret_number);

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You gussed: {}", guess);
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // match guess.cmp(&secret_number) {
    //     Ordering::Less => println!("Too small!"), 
    //     Ordering::Greater  => println!("Too big!"),
    //     Ordering::Equal => println!("You win!"),
    // }
    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

    // println!("{}", five_hundred);

    // let a : [i32;6] = [20,30,40,50,60,70];
    // println!("{}", a );

    let a = [1,2,3,4,5,6];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("THe value of the element at index {index} is   {element}" );

    let y = plus_one(5);
    println!("THe value of the element at index {y} is")
}

fn plus_one(x:i32) -> i32 {
    x+1
}