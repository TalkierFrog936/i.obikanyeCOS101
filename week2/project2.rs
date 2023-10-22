fn main() {
	let t:f64 = 900000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 2250000.00;
	let d:f64 = 8550000.00;
	let a:f64 = 250000.00;

    let sum:f64 = t + m + h + d+ a;
    println!("sum is {}",sum ); 
    let average:f64 = sum / 10.00;
    println!("average is {}",average );
}