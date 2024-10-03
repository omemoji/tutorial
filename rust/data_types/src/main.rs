use std::io;
fn main() {
    // no type annotation
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);

    // float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {}, y: {}", x, y);

    // calculation

    let sum = 5 + 10;
    println!("sum: {}", sum);

    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    let product = 4 * 30;
    println!("product: {}", product);

    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    println!("quotient: {}", quotient);
    println!("floored: {}", floored);

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("t: {}, f: {}", t, f);

    // character

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // compound types

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!(
        "five_hundred: {}, six_point_four: {}, one: {}",
        five_hundred, six_point_four, one
    );

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months: {:?}", months);

    let a = [3; 5];
    println!("a: {:?}", a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

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

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
