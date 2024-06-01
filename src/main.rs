use std::io;

pub fn adder(x: i32, y: i32) -> i32 {
    let z = x + y;
    z
}

pub fn subber(x: i32, y: i32) -> i32 {
    let z = x - y;
    z
}

pub fn multer(x: i32, y: i32) -> i32 {
    let z = x * y;
    z
}

pub fn divider(x: i32, y: i32) -> i32 {
    let z = x / y;
    z
}

pub fn reterror(x: i32, y: i32) -> i32 {
    let z = 0;
    z
}

fn main() {
    let mut inp_a = '1';
    let mut inp_b = '1';
    let mut inp_c = 'c';
    let mut inp = String::new();
    
    let mut c = 0;
    println!("Welcome to the calculator!");
    println!("Enter a binary expression without spaces: ");

    io::stdin().read_line(&mut inp).expect("Failed to read line");

    let my_vec: Vec<char> = inp.chars().collect();

    inp_a = my_vec[0];
    let a: i32 = inp_a as i32 - 0x30;
    
    inp_b = my_vec[2];
    let b: i32 = inp_b as i32 - 0x30;

    inp_c = my_vec[1];
    
    let result = match inp_c {
        '+' => adder(a,b),
        '-' => subber(a,b),
        '*' => multer(a,b),
        '/' => divider(a,b),
        _ => reterror(a,b), 
    };

    println!("The result of the expression is: {result}");

}
