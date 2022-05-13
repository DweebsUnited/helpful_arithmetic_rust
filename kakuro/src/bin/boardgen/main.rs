use kakuro;

fn main( ) {

	let s = kakuro::board::generation::CreationSettings{ width: 5, height: 5, method: kakuro::board::generation::CreationMethod::Spiralfill { cap_prob: 0.33, start_cap: true } };

	let b = kakuro::board::Board::new( s );

	println!( "{}", b );

	let s = kakuro::board::generation::CreationSettings{ width: 5, height: 5, method: kakuro::board::generation::CreationMethod::Segment {  } };

	let b = kakuro::board::Board::new( s );

	println!( "{}", b );

}