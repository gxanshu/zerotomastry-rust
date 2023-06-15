// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum DrinkFlavors {
	Tasty,
	Nasty
}

struct Drinks {
	falvor: DrinkFlavors,
	quantity: i32,
};

fn print_drinks(drink: Drinks){
	match Drink.falvor {
		Drinks::Tasty => println!("tasty"),
		Drinks::Nasty => println!("Nasty"),
	}
	println!("{:?}", drink.quantity);
}

fn main() {
	let cococola = Drinks {
		falvor: DrinkFlavors::Tasty,
		quantity: 23
	}

	print_drinks(cococola);
}
