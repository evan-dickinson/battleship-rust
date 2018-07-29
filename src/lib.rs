// rustc --crate-type lib --emit llvm-ir lib.rs -O

pub mod client;
pub mod network;

pub enum Hat {
	Bowler,
	Ball,
	Fedora,
	Fancy(u64)
}

pub fn hat_cost(hat : &Hat) -> u64 {
	match *hat {
		Hat::Bowler => 500,
		Hat::Ball   => 5,
		Hat::Fedora => 0,
		Hat::Fancy(cost) => cost,
	}
}

pub fn hats_cost(hats: &[Hat]) -> u64 {
	return hats.iter()
		.map(hat_cost)
		.sum();
}


pub fn describe_hat(hat : Hat) -> & 'static str {
	match hat {
		Hat::Bowler => "A fine hat",
		Hat::Ball   => "Bleh",
		Hat::Fedora => "No!",
		Hat::Fancy(_)  => "So fancy!"
	}
}

pub fn foo() -> Option<i32> {
	let a = [1, 2, 3, 4];
	let b = a.iter()
		.map(|x| { x * 2 })
		.nth(2);
		//.unwrap();

	return b;
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
