use std::fmt::Display;

use crate::rule::*;
use crate::cell::*;

pub mod generation;
use generation::*;

use rand::Rng;

// This is an arena style allocation of board elements
pub struct Board {

	width: usize,
	height: usize,

	rules: Vec<Rule>,
	cells: Vec<Cell>,

}


impl Board {

	// I want to implement several algorithms and settings for
	pub fn new( settings: CreationSettings ) -> Self {

		match settings.method {
			CreationMethod::Spiralfill{ cap_prob } => {
				Board::new_spiralfill( settings.width, settings.height, cap_prob )
			},
			CreationMethod::Floodfill{ } => {
				Board::new_floodfill( settings.width, settings.height )
			},
			CreationMethod::Segment{  } => {
				Board::new_segment( settings.width, settings.height )
			},
			_ => {
				Board::new_empty( settings.width, settings.height )
			}
		}
	}

	// Create a new board by a spiral fill algorithm
	// This will result in a nearly completely filled grid
	fn new_spiralfill( width: usize, height: usize, cap_prob: f32 ) -> Self {

		// Stage 1 -- Spiral fill
		// Run in a spiral centered on middle ( default up and left if even grid size )
		//   If first cell, set appropriately
		//   If on the shell, force to cap
		//   Otherwise pick type randomly
		//   Set value valid if so picked, cap otherwise

		// Stage 2 -- Cap -> Rule
		// For each cap in the grid
		//   If it has no numeric neighbors, change to unset, skip
		//   For each direction
		//     Find matching cap
		//     Update cap links
		//     Check if matching cap already has a Rule, link if so
		//     If not, make a new rule and link us and matching

		let mut b: Board = Board::new_empty( width, height );

		let spiral = crate::utility::SpiralGenerator::new( width, height );

		let mut rng: rand::rngs::ThreadRng = rand::thread_rng( );
		let type_dist = rand::distributions::Bernoulli::new( cap_prob as f64 ).unwrap( );
		let digit_dist = rand::distributions::Uniform::new_inclusive( 0 as u8, 9 as u8 );

		for coord in spiral {

			if rng.sample( type_dist ) || ( coord.0 == 0 || coord.0 == width - 1 ) || ( coord.1 == 0 || coord.1 == height - 1 ) {
				b.cells.push( Cell::Cap( CellCap::new( ) ) );
			} else {
				b.cells.push( Cell::Value( CellValue::new( ) ) );
			}
			b.cells.swap_remove( coord.0 + coord.1 * width );

		}

		b

	}

	fn new_floodfill( width: usize, height: usize ) -> Self {
		unimplemented!( );
	}

	fn new_segment( width: usize, height: usize ) -> Self {
		todo!( );
	}

	fn new_empty( width: usize, height: usize ) -> Self {
		let mut b = Board {
			width,
			height,
			rules: Vec::new( ),
			cells: Vec::new( )
		};

		for _ in 0..( width * height ) {
			b.cells.push( Cell::Unused );
		}

		b

	}

}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print header
		write!( f, "*-" )?;
		for _ in 0..self.width {
			write!( f, "--" )?;
		}
		write!( f, "*\n" )?;

		// For each row
		for r in 0..self.height {
			write!( f, "| " )?;

			for c in 0..self.width {

				write!( f, "{} ", &self.cells[ c + r * self.width ] )?;

			}

			write!( f, "|\n" )?;
		}

		// Print footer
		write!( f, "*-" )?;
		for _ in 0..self.width {
			write!( f, "--" )?;
		}
		writeln!( f, "*\n" )?;
		Ok(())
    }
}