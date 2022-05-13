use std::fmt::Display;

pub struct CellCap {
	rule_up: Option<usize>,
	rule_down: Option<usize>,
	rule_left: Option<usize>,
	rule_right: Option<usize>
}

pub struct CellValue {
	val: Option<u8>,
	rule: Option<usize>
}

// Cells can be solution caps, or numerics
pub enum Cell {

	Cap( CellCap ),
	Value( CellValue ),

	Unused

}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		match self {
			Cell::Cap( c ) => {
				write!( f, "X" )
			},
			Cell::Value( v ) => {
				match v.val {
					Some( digit ) => {
						write!( f, "{}", digit )
					},
					None => {
						write!( f, "_" )
					}
				}
			},
			Cell::Unused => {
				write!( f, " " )
			}
		}

    }
}