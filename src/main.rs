struct Student <'a> {
    roll_no: u8,
    name: &'a str,
    age: u8,
}

fn main(){
    let mut x = 5;
    x += 3;
    let y = 6;
    let z = x + y;
    println!("Sum of nos is {}", z);

fn next_birthday(name: &str, age: u8) {
    let next_age = age + 1;
    println!("Hi {},On next birthday you will be {}",name, next_age);
}

fn square_maker(number: u8) -> u8{number * number}

let ramesh = Student{name: "Ramesh", age: 21, roll_no: 1};

next_birthday(ramesh.name, ramesh.age);

next_birthday("Umesh", 27);

square_maker(1);
square_maker(11);
next_birthday("Utkarsh", square_maker(2));

}
