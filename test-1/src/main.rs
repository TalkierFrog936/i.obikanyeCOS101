//Rust program for patient info

use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();

	println!("Enter your name: ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("Enter your age: ");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let age:i32 = input2.trim().parse().expect("Not a valid number");


	println!("Enter your email address: ");
	io::stdin().read_line(&mut input3).expect("Not a valid string");


   println!("Enter your phone number");
   io::stdin().read_line(&mut input4).expect("Not a valid string");

   println!("Enter number of siblings: ");
   io::stdin().read_line(&mut input5).expect("Not a valid string");
   let number of siblings:i32 = input5.trim().parse().expect("Not a valid number");

   println!("Enter number of children: ");
   io::stdin().read_line(&mut input6).expect("Not a valid string");
   let number of children:i32 = input6.trim().parse().expect("Not a valid number");

   println!("Enter Health diagnosis");
   io::stdin().read_line(&mut input7).expect("Not a valid string");
   let Health diagnosis:f32 = input7.trim().parse().expect("Not a valid string");

   println!("Enter your village of residence");
   io::stdin().read_line(&mut input8).expect("Not a valid string");
   let village of residence:f32 = input8.trim().parse().expect("Not a valid string");

   if age > 50 && number of children > 4 && Health diagnosis = Alzheimer && village of residence = Akpabom:

}


