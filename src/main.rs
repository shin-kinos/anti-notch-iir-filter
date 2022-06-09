
//#[macro_use]
//extern crate peroxide;
//use peroxide::fuga::*;

struct Filter {
	x1 : f32,
	x2 : f32,
	y1 : f32,
	y2 : f32,
}

impl Filter {
	fn new() -> Filter {
		let x1 = 0.0;
		let x2 = 0.0;
		let y1 = 0.0;
		let y2 = 0.0;

		Filter {
			x1 : x1,
			x2 : x2,
			y1 : y1,
			y2 : y2,
		}
	}

	fn run( &mut self, b : &Vec<f32>, a : &Vec<f32>, input_vec : &Vec<f32> ) -> Vec<f32> {

		let mut output_vec : Vec<f32> = Vec::new();
		let vec_len : usize = ( *input_vec ).len();

		for i in 0 .. vec_len {
			let input_val : f32 = ( *input_vec )[ i ];

			let output_val : f32 =
				( ( *b )[ 0 ] * input_val ) +
				( ( *b )[ 1 ] * self.x1   ) +
				( ( *b )[ 2 ] * self.x2   ) -
				( ( *a )[ 1 ] * self.y1   ) -
				( ( *a )[ 2 ] * self.y2   );

			self.x2 = self.x1;
			self.x1 = input_val;
			self.y2 = self.y1;
			self.y1 = output_val;

			output_vec.push( output_val );
		}

		output_vec

	}
}

fn main() {

	let pi    : f32 = std::f32::consts::PI;
	let r     : f32 = 0.992;
	let theta : f32 = 2.0 * pi / 3.0;

	let b : Vec<f32> = vec![ 1.0, 0.0, -1.0 ];
	let a : Vec<f32> = vec![ 1.0, -2.0 * r * theta.cos(), r * r ];

	let input_vec = vec![ 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0 ];

	let mut filter = Filter::new();

	let output_vec : Vec<f32> = filter.run( &b, &a, &input_vec );

	for elem in output_vec {
		println!( "{:.3}", elem );
	}

}
