use sequencematcher::SequenceMatcher;


pub struct Differ {
    linejunk: Option<String>,
    charjunk: Option<String>
}

impl Differ{
	pub fn new() -> Differ {
		Differ{
			linejunk: None,
			charjunk: None
		}
	}
}