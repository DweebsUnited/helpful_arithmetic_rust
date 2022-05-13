use kakuro;

fn main( ) {

	let s = kakuro::board::generation::CreationSettings{ width: 15, height: 15, method: kakuro::board::generation::CreationMethod::Spiralfill { cap_prob: 0.175 } };

	let b = kakuro::board::Board::new( s );

	println!( "{}", b );

	let s = kakuro::board::generation::CreationSettings{ width: 9, height: 9, method: kakuro::board::generation::CreationMethod::Segment { } };

	let b = kakuro::board::Board::new( s );

	println!( "{}", b );

}