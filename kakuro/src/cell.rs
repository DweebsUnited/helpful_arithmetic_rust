use std::fmt::Display;

// Cells can have a few types
pub enum Cell {

	Cap( CellCap ),
	Value( CellValue ),

	Unused

}

pub struct CellCap {
	rule_up: Option<usize>,
	rule_down: Option<usize>,
	rule_left: Option<usize>,
	rule_right: Option<usize>
}
impl CellCap {
	pub fn new( ) -> Self {
		Self{
			rule_up: None,
			rule_down: None,
			rule_left: None,
			rule_right: None }
	}
	pub fn set_up( s: &Self, r: Option<usize> ) -> Self {
		Self{
			rule_up: r,
			rule_down: s.rule_down.clone( ),
			rule_left: s.rule_left.clone( ),
			rule_right: s.rule_right.clone( ) }
	}
	pub fn set_down( s: &Self, r: Option<usize> ) -> Self {
		Self{
			rule_up: s.rule_up.clone( ),
			rule_down: r,
			rule_left: s.rule_left.clone( ),
			rule_right: s.rule_right.clone( ) }
	}
	pub fn set_left( s: &Self, r: Option<usize> ) -> Self {
		Self{
			rule_up: s.rule_up.clone( ),
			rule_down: s.rule_down.clone( ),
			rule_left: r,
			rule_right: s.rule_right.clone( ) }
	}
	pub fn set_right( s: &Self, r: Option<usize> ) -> Self {
		Self{
			rule_up: s.rule_up.clone( ),
			rule_down: s.rule_down.clone( ),
			rule_left: s.rule_left.clone( ),
			rule_right: r }
	}
}

pub struct CellValue {
	val: Option<u8>,
	rule: Option<usize>
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