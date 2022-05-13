pub struct Rule {

	pub total: u8,
	pub combos: Vec<Vec<u8>>

}

impl Rule {

    pub fn new( total: u8 ) -> Self {
		let possibles = build_possibles( total );

		Self { total, combos: possibles }
	}

}

fn build_possibles( total: u8 ) -> Vec<Vec<u8>> {

	let mut possible_stack: Vec<u8> = Vec::new( );
	let mut possibles: Vec<Vec<u8>> = Vec::new( );

	recurse_possibles( total, &mut possible_stack, 0, 1, &mut possibles );

	possibles

}

fn recurse_possibles( total: u8, stack: &mut Vec<u8>, running_total: u8, startfrom: u8, collection: &mut Vec<Vec<u8>> ) {

	// print!( "Stack: " );
	// for d in stack.iter( ) {
	// 	print!( "{}", d );
	// }
	// println!( );

	// Assume we haven't exceeded total already
	// assert!( running_total < total );

	// For remaining digits up to 9, try adding to the solution
	// If startfrom > 9, this does nothing
	for digit in startfrom..10 {

		stack.push( digit );
		let running_total = running_total + digit;

		// println!( "{}:{}:{:?}", running_total, stack.len( ), stack );

		// If we find a solution, save it
		if running_total == total {
			collection.push( stack.to_vec( ) );
		}

		// Only carry on until we exceed the total
		if running_total < total {

			// Carry on recursing if we reach here!
			recurse_possibles( total, stack, running_total, digit + 1, collection );

		}

		// Don't forget to clean up the stack
		stack.pop( );

	}

}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!( f, "{:>2}:{}: ", self.total, self.combos.len( ) )?;
		for c in self.combos.iter( ) {
			for digit in c {
				write!( f, "{}", digit )?;
			}
			write!( f, " " )?;
		}
		Ok(())
    }
}