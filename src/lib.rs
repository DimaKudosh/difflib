pub mod sequencematcher;
pub mod differ;
mod utils;


use sequencematcher::{SequenceMatcher, Sequence};
use utils::{format_range_unified};


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

pub fn unified_diff<T: Sequence>(first_sequence: &T, second_sequence: &T, from_file: &str, to_file: &str, 
	from_file_date: &str, to_file_date: &str, n: usize, lineterm: &str) -> Vec<String> {
	let mut res = Vec::new();
	let mut started = false;
	let mut matcher = SequenceMatcher::new(first_sequence, second_sequence);
	for group in &matcher.get_grouped_opcodes(n) {
		if !started {
			started = true;
			let from_date = format!("\t{}", from_file_date);
			let to_date = format!("\t{}", to_file_date);
			res.push( format!("--- {}{}{}", from_file, from_date, lineterm) );
			res.push( format!("+++ {}{}{}", to_file, to_date, lineterm) );
		}
		let (first, last) = (group.first().unwrap(), group.last().unwrap());
		let file1_range = format_range_unified(first.first_start, last.first_end); 
		let file2_range = format_range_unified(first.second_start, last.second_end);
		res.push( format!("@@ -{} +{} @@{}", file1_range, file2_range, lineterm) );
		for code in group {
			if code.tag == "equal" {
				for i in code.first_start..code.first_end {
					res.push( format!(" {}", first_sequence.at_index(i).unwrap()) );
				}
				continue;
			}
			if code.tag == "replace" || code.tag == "delete" {
				for i in code.first_start..code.first_end {
					res.push( format!("-{}", first_sequence.at_index(i).unwrap()) );
				}
			}
			if code.tag == "replace" || code.tag == "insert" {
				for i in code.second_start..code.second_end {
					res.push( format!("+{}", second_sequence.at_index(i).unwrap()) );
				}
			}
		}
	}
	res
}
