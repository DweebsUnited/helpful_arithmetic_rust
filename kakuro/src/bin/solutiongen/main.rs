// Generate rule possibilities for each answer value

use kakuro;

fn main( ) {

	let maxsolution: u8 = ( 1..10 ).sum( );
	for solution in ( 1 as u8 )..( maxsolution + 1 ) {

		let r = kakuro::rule::Rule::new( solution );
		println!( "{}", r );

	}

}