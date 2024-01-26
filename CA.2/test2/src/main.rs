//importing function crates
use std::io;
use std::fs::Files;
use std::io::write;
fn main() {
	struct Company {
		shares:i32,
		liabilities:i32,
		founded:i32
		username:String,
		password String,
	}
	impl Company{
		fn percentage_leverages(&self)->f32{
			return self.liabilities / self.shares * 100.0;
		}
	}
	    impl Company {
	    	// add code here
	    }
}