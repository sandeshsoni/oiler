struct Student {
    roll_no: u8,
    // name: &'a str,
    name: String,
    age: u8,
}

fn main(){
    let mut x = 5;
    x += 3;
    let y = 6;
    let z = x + y;
    println!("Sum of nos is {}", z);

fn next_birthday(name: String, age: u8) {
    let next_age = age + 1;
    println!("Hi {},On next birthday you will be {}",name, next_age);
}

fn square_maker(number: u8) -> u8{number * number}

// ownership

let mut greet = String::from("hello");
greet.push_str(" there!");
println!("I say, {}", greet);

let utk = String::from("Utkarsh");
next_birthday(utk, square_maker(2));

// struct

let ramesh = Student{
    name: String::from("Ramesh"),
    age: 21,
    roll_no: 1
    };
next_birthday(ramesh.name, ramesh.age);

// ownership name change
let name = String::from("Umesh");

// add clone and make it run
next_birthday(name.clone(), 27);
next_birthday(name, 28);

square_maker(1);
square_maker(11);

fn pluralise(mut input_word: String) -> String {
   input_word.push_str("s");
   return input_word;
}

// with borrowing concept
fn pluralise_with_borrow(input_word: &str) -> String {
    input_word.to_owned() + "s"
}

// fn to pluralise input
let word = String::from("book");
println!("I have one {}, you have two {}",
    word.clone(),
    pluralise(word)
);

let word2 = String::from("pen");
println!("I have one {}, you have two {}",
    word2,
    pluralise_with_borrow(&word2)
);


}
