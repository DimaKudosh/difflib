pub mod sequencematcher;
pub mod differ;
mod utils;


use sequencematcher::SequenceMatcher;


pub fn get_close_matches<'a>(word: &str, possibilities: Vec<&'a str>, n: usize, cutoff: f32) -> Vec<&'a str> {
	if !(0.0 <= cutoff && cutoff <= 1.0) {
		//error
	}
	let mut res: Vec<(f32, &str)> = Vec::new();
	let mut matcher = SequenceMatcher::new("", word);
	for i in &possibilities {
		matcher.set_first_seq(i);
		let ratio = matcher.ratio();
		if ratio >= cutoff {
			res.push((ratio, i));
		}
	}
	res.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
	res.truncate(n);
	res.iter().map(|x| x.1).collect()
}