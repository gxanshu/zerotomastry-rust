// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
	let my_num = 4;
	match my_num {
		1 => println!("1"),
		2 => println!("2"),
		3 => println!("3"),
		_ => println!("{:?}", my_num);
	}
}
