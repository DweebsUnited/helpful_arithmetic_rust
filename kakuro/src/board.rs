use std::fmt::Display;

use crate::rule::*;
use crate::cell::*;

pub mod generation;
use generation::*;

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
			CreationMethod::Spiralfill{ cap_prob, start_cap } => {
				Board::new_spiralfill( settings.width, settings.height, cap_prob, start_cap )
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
	fn new_spiralfill( width: usize, height: usize, cap_prob: f32, start_cap: bool ) -> Self {

		// Stage 1 -- Spiral fill
		// Run in a spiral centered on middle ( default up and left if even grid size )
		//   If first cell, set appropriately
		//   Determine type
		//   Set value valid if so picked, cap otherwise
		//   Add unset neighbors to working list

		// Stage 2 -- Cap -> Rule
		// For each cap in the grid
		//   If it has no numeric neighbors, it needs to stay unset, skip
		//   For each direction
		//     Find matching cap
		//     Update cap links
		//     Check if matching cap already has a Rule, link if so
		//     If not, make a new rule and link us and matching

		let mut b: Board = Board::new_empty( width, height );

		let spiral = crate::utility::SpiralGenerator::new( width, height );
		let mut first = true;

		for coord in spiral {

			println!( "{}x{}", coord.0, coord.1 );

			// if first {

			// 	if start_cap {

			// 		b.cells.push( Cell::Cap( Cell::CellCap::new( ) ) );

			// 	} else {

			// 		b.cells.push( );

			// 	}

			// 	b.cells.swap_remove( coord.0 + coord.1 * width );

			// 	first = false;

			// }

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