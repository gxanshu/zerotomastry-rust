// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
	Red,
	Blue
}

struct Box {
	dimensions: i32,
	weight: i32,
	color: Color
}

impl Box {
	fn create(dimensions: i32, weight: i32, color: Color) -> Self {
		Self {
			dimensions,
			weight,
			color
		}
	}

	fn print(&self) {
		println!("{:?} {:?}", self.dimensions, self.weight);
		match self.color {
			Color::Red => println!("red"),
			Color::Blue => println!("Blue"),
		}
	}
}

fn main() {
	let apple = Box::create(12, 5, Color::Red);
	apple.print();
}
