 fn main(){
 	let p:f64 = 520000000.00;
 	let r:f64 = 10.00;
 	let n:f64 = 5.00;

 	let a:f64 = p * (1.00 + (r/100.00)) * n;
 	let c:f64 = a - p;

 	println!("The compound interest = {}",c);
 }
 
	
