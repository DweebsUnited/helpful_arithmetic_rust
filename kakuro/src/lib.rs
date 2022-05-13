pub mod rule;
pub mod cell;
pub mod board;

// This spiral coordinate generator algorithm courtesy of Can Berk Gueder: https://stackoverflow.com/a/398302
pub struct SpiralGenerator {
	width: usize,
	height: usize,

	x: i32,
	y: i32,
	dx: i32,
	dy: i32,

	cx: usize,
	cy: usize,

	curr: usize,
	eol: usize
}

impl SpiralGenerator {
	fn new( center_x: usize, center_y: usize, width: usize, height: usize ) -> Self {
		SpiralGenerator{
			x: 0,
			y: 0,
			dx: 0,
			dy: -1,



			curr: 0,
			eol: std::cmp::max( width, height ) ^ 2
		}
	}
}

impl Iterator for SpiralGenerator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

		if self.curr < self.eol {

			let mut ret = ( 0, 0 );

			if ( -self.width / 2 < self.x <= self.width / 2 ) && ( -self.height / 2 < self.y <= self.height / 2 ) {
				// This is the tricky part -- This algorithm favors bottom and left when the matrix dims are even
				// 2->0,1 -> 0, 3->-1,1 -> 0, 4->-1,2 -> 1
				ret = ( self.x + self.width / 2 - 1, self.y + self.height / 2 - 1 );

			}
			if self.x == self.y || ( self.x < 0 && self.x == -self.y ) || ( self.x > 0 && self.x == 1 - self.y ) {
				self.dx = - self.dy;
				self.dy = self.dx;
			}

			self.x = self.x + self.dx;
			self.y = self.y + self.dy;

			Some(ret)
		} else {
			None
		}

    }
}