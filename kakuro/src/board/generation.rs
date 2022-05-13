pub enum CreationMethod {
	Spiralfill{ cap_prob: f32 },
	Floodfill{ },
	Segment{ },

	Empty
}

pub struct CreationSettings {
	pub width: usize,
	pub height: usize,
	pub method: CreationMethod
}