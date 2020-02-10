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

next_birthday("Ramesh", 21);
next_birthday("Umesh", 27);

}
