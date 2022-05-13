// This spiral coordinate generator algorithm courtesy of Can Berk Gueder: https://stackoverflow.com/a/398302
#[derive(Debug)]
pub struct SpiralGenerator {
	width: usize,
	height: usize,

	x: i32,
	y: i32,
	dx: i32,
	dy: i32,

	curr: usize,
	eol: usize
}

impl SpiralGenerator {
	pub fn new( width: usize, height: usize ) -> Self {
		SpiralGenerator{
			width,
			height,

			x: 0,
			y: 0,
			dx: 0,
			dy: -1,

			curr: 0,
			eol: std::cmp::max( width, height ).pow( 2 )
		}
	}
}

impl Iterator for SpiralGenerator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

		println!( "{} < {}, {}", self.curr, self.eol, self.width as i32 / 2 );
		println!( "{:?}", self );

		if self.curr < self.eol {

			println!( "{}<{}<={} - {}<{}<={} -- {}x{}",
				-( self.width as i32 ) / 2, self.x, ( self.width as i32 ) / 2,
				-( self.height as i32 ) / 2, self.y, ( self.height as i32 ) / 2,
				( self.x + ( self.width / 2 - 1 ) as i32 ) as usize, ( self.y + ( self.height / 2 - 1 ) as i32 ) as usize );

			let mut ret = ( 0, 0 );

			if ( -( self.width as i32 ) / 2 < self.x && self.x <= ( self.width as i32 ) / 2 ) &&
				( -( self.height as i32 ) / 2 < self.y && self.y <= ( self.height as i32 ) / 2 ) {
				// This is the tricky part -- This algorithm favors bottom and left when the matrix dims are even
				// 2->0,1 -> 0, 3->-1,1 -> 0, 4->-1,2 -> 1
				ret = ( ( self.x + ( self.width / 2 ) as i32 ) as usize, ( self.y + ( self.height / 2 ) as i32 ) as usize );

			}

			if self.x == self.y || ( self.x < 0 && self.x == -self.y ) || ( self.x > 0 && self.x == 1 - self.y ) {
				let t = self.dx;
				self.dx = - self.dy;
				self.dy = t;
			}

			self.x = self.x + self.dx;
			self.y = self.y + self.dy;

			self.curr += 1;

			Some(ret)
		} else {
			None
		}

    }
}