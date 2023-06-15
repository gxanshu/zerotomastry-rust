// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
	quantity: i32,
	id: i32
}

fn print_quantity(apple: &Grocery) {
	println!("{:?}", apple.quantity);
}

fn print_id(apple: &mut Grocery) {
	println!("{:?}", apple.id);
	apple.id = 2;
}

fn main() {
	let mut apple = Grocery {
		quantity: 5,
		id: 1
	};
	print_quantity(&apple);
	print_id(&mut apple);
	println!("new item = {:?}", apple.id);
}
