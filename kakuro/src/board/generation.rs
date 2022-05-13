pub enum CreationMethod {
	Spiralfill{ cap_prob: f32, start_cap: bool },
	Floodfill{ },
	Segment{ },

	Empty
}

pub struct CreationSettings {
	pub width: usize,
	pub height: usize,
	pub method: CreationMethod
}