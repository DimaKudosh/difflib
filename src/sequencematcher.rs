use std::collections::HashMap;
use std::cmp::{Ordering, max, min};
use utils::calculate_ratio;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Match{
	pub first_start: usize,
	pub second_start: usize,
	pub size: usize
}

impl Match{
	fn new(first_start: usize, second_start: usize, size: usize) -> Match {
		Match{
			first_start: first_start, 
			second_start: second_start,
			size: size
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Opcode{
	pub tag: String,
	pub first_start: usize,
	pub first_end: usize,
	pub second_start: usize,
	pub second_end: usize
}

impl Opcode{
	fn new(tag: String, first_start: usize, first_end: usize, second_start: usize, second_end: usize) -> Opcode {
		Opcode{
			tag: tag,
			first_start: first_start,
			first_end: first_end,
			second_start: second_start,
			second_end: second_end
		}
	}
}

pub trait Sequence: Debug {
	fn len(&self) -> usize;
	fn at_index(&self, index: usize) -> Option<&str>;
}

impl Sequence for str {
	fn len(&self) -> usize {
		self.len()
	}

	fn at_index(&self, index: usize) -> Option<&str> {
		if index > self.len(){
			return None
		}
		unsafe{
			Some(self.slice_unchecked(index, index + 1))
		}
	}
}

impl<'a> Sequence for Vec<&'a str> {
	fn len(&self) -> usize {
		self.len()
	}

	fn at_index(&self, index: usize) -> Option<&str> {
		if index < self.len() && index >= 0 {
			return Some(self[index])
		}
		None
	}
}

impl<'a> Sequence for [&'a str] {
	fn len(&self) -> usize {
		self.len()
	}

	fn at_index(&self, index: usize) -> Option<&str> {
		if index < self.len() && index >= 0 {
			return Some(self[index])
		}
		None
	}
}

pub struct SequenceMatcher<'a, T: 'a + ?Sized + Sequence>{
	first_sequence: &'a T,
	second_sequence: &'a T,
	matching_blocks: Option<Vec<Match>>,
	opcodes: Option<Vec<Opcode>>,
	is_junk: Option<fn(&str) -> bool>,
	second_sequence_elements: HashMap<&'a str, Vec<usize>>,
	second_sequence_popular: Vec<&'a str>
}


impl<'a, T: ?Sized + Sequence> SequenceMatcher<'a, T>{
	pub fn new(first_sequence: &'a T, second_sequence: &'a T) -> SequenceMatcher<'a, T> {
		let mut matcher = 
		SequenceMatcher{
			first_sequence: first_sequence,
			second_sequence: second_sequence,
			matching_blocks: None,
			opcodes: None,
			is_junk: None,
			second_sequence_elements: HashMap::new(),
			second_sequence_popular: Vec::new()
		};
		matcher.set_seqs(first_sequence, second_sequence);
		matcher
	}

	pub fn set_is_junk(&mut self, is_junk: Option<fn(&str) -> bool>) {
		self.is_junk = is_junk;
		self.set_second_seq(self.second_sequence);
	}

	pub fn set_seqs(&mut self, first_sequence: &'a T, second_sequence: &'a T) {
		self.set_first_seq(first_sequence);
		self.set_second_seq(second_sequence);
	}

	pub fn set_first_seq(&mut self, sequence: &'a T) {
		self.first_sequence = sequence;
		self.matching_blocks = None;
		self.opcodes = None;
	}

	pub fn set_second_seq(&mut self, sequence: &'a T) {
		self.second_sequence = sequence;
		self.matching_blocks = None;
		self.opcodes = None;
		self.chain_second_seq();
	}
	
	fn chain_second_seq(&mut self) {
		let second_sequence = self.second_sequence;
		let mut second_sequence_elements = HashMap::new();
		for i in 0..second_sequence.len() {
			let mut counter = second_sequence_elements.entry(second_sequence.at_index(i).unwrap()).or_insert(Vec::new());
			counter.push(i);
		}
		if self.is_junk.is_some() {
			let mut junk = Vec::new();
			for element in second_sequence_elements.keys() {
				if (self.is_junk.unwrap())(element) {
					junk.push(element.clone());
				}
			}
			for element in &junk {
				second_sequence_elements.remove(element);
			}
		}
		let mut popular = Vec::new();
		let len = second_sequence.len();
		if len >= 200 {
			let test_len = (len as f32 / 100.0).floor() as usize + 1;
			for (element, indexes) in second_sequence_elements.iter() {
				if indexes.len() > test_len {
					popular.push(element.clone());
				}
			}
			for element in &popular {
				second_sequence_elements.remove(element);
			}
		}
		self.second_sequence_elements = second_sequence_elements;
		self.second_sequence_popular = popular;
	}

	pub fn find_longest_match(&self, first_start: usize, first_end: usize, second_start: usize, second_end: usize) -> Match { 
		let first_sequence = &self.first_sequence;
		let second_sequence = &self.second_sequence;
		let second_sequence_elements = &self.second_sequence_elements;
		let (mut best_i, mut best_j, mut best_size) = (first_start, second_start, 0);
		let mut j2len: HashMap<usize, usize> = HashMap::new();
		for i in first_start..first_end {
			let mut new_j2len: HashMap<usize, usize> = HashMap::new();
			match second_sequence_elements.get(first_sequence.at_index(i).unwrap()) {
				Some(indexes) => {
					for j in indexes {
						let j = j.clone();
						if j < second_start {
							continue;
						};
						if j >= second_end {
							break;
						};
						let mut size = 0;
						if j > 0 {
							match j2len.get(&(j-1)){
								Some(k) => {
									size = k.clone();
								},
								None => {}
							}
						}
						size += 1;
						new_j2len.insert(j, size);
						if size > best_size {
							best_i = i + 1 - size;
							best_j = j + 1 - size;
							best_size = size;
						}
					}
				},
				None => {},
			}
			j2len = new_j2len;
		}
		for _ in 0..2 {
			while best_i > first_start && best_j > second_start && 
			first_sequence.at_index(best_i - 1) == second_sequence.at_index(best_j-1){
				best_i = best_i - 1;
				best_j = best_j - 1;
				best_size = best_size + 1;
			}
			while best_i + best_size < first_end && best_j + best_size < second_end &&
			first_sequence.at_index(best_i + best_size) == second_sequence.at_index(best_j + best_size) {
				best_size += 1;
			}
		}
		Match::new(best_i, best_j, best_size)
	}

	pub fn get_matching_blocks(&mut self) -> Vec<Match> {
		if self.matching_blocks.as_ref().is_some(){
			return self.matching_blocks.as_ref().unwrap().clone()
		}
		let (first_length, second_length) = (self.first_sequence.len(), self.second_sequence.len());
		let mut matches = Vec::new();
		let mut queue = vec![(0, first_length, 0, second_length)];
		while !queue.is_empty(){
			let (first_start, first_end, second_start, second_end) = queue.pop().unwrap();
			let m = self.find_longest_match(first_start, first_end, second_start, second_end);
			match m.size {
				0 => {},
				_ => {
					if first_start < m.first_start && second_start < m.second_start{
						queue.push((first_start, m.first_start, second_start, m.second_start));
					}
					if m.first_start + m.size < first_end && m.second_start + m.size < second_end{
						queue.push((m.first_start + m.size, first_end, m.second_start + m.size, second_end));
					}
					matches.push(m);
				},
			}
		}
		matches.sort_by(|a, b| a.cmp(b));
		let (mut first_start, mut second_start, mut size) = (0, 0, 0);
		let mut non_adjacent = Vec::new();
		for m in &matches{
			if first_start + size == m.first_start && second_start + size == m.second_start{
				size += m.size
			}
			else {
				if size != 0{
					non_adjacent.push(Match::new(first_start, second_start, size));
				}
				first_start = m.first_start;
				second_start = m.second_start;
				size = m.size;
			}
		}	
		if size != 0{
			non_adjacent.push(Match::new(first_start, second_start, size));
		}
		non_adjacent.push(Match::new(first_length, second_length, 0));
		self.matching_blocks = Some(non_adjacent);
		self.matching_blocks.as_ref().unwrap().clone()
	}

	pub fn get_opcodes(&mut self) -> Vec<Opcode>{
		if self.opcodes.as_ref().is_some(){
			return self.opcodes.as_ref().unwrap().clone()
		}
		let mut opcodes = Vec::new();
		let (mut i, mut j) = (0, 0);
		for m in self.get_matching_blocks(){
			let mut tag = String::new();
			if i < m.first_start && j < m.second_start{
				tag = String::from("replace");
			}
			else if i < m.first_start{
				tag = String::from("delete");
			}
			else if j < m.second_start{
				tag = String::from("insert");
			}
			if !tag.is_empty(){
				opcodes.push( Opcode::new(tag, i, m.first_start, j, m.second_start) );
			}
			i = m.first_start + m.size;
			j = m.second_start + m.size;
			if m.size != 0{
				opcodes.push( Opcode::new(String::from("equal"), m.first_start, i, m.second_start, j) );
			}
		}
		self.opcodes = Some(opcodes);
		return self.opcodes.as_ref().unwrap().clone()
	}

	pub fn get_grouped_opcodes(&mut self, n: usize) -> Vec<Vec<Opcode>> {
		let mut res = Vec::new();
		let mut codes = self.get_opcodes();
		if codes.is_empty() {
			codes.push(Opcode::new("equal".to_string(), 0, 1, 0, 1));
		}

		if codes.first().unwrap().tag == "equal" {
			let mut opcode = codes.first_mut().unwrap();
			opcode.first_start = max(opcode.first_start, opcode.first_end - n);
			opcode.second_start = max(opcode.second_start, opcode.second_end - n);
		}
		if codes.last().unwrap().tag == "equal" {
			let mut opcode = codes.last_mut().unwrap();
			opcode.first_end = min(opcode.first_start + n, opcode.first_end);
			opcode.second_end = min(opcode.second_start + n, opcode.second_end);
		}
		let nn = n + n;
		let mut group = Vec::new();
		for code in &codes {
			let (mut first_start, mut second_start) = (code.first_start, code.second_start);
			if code.tag == "equal" && code.first_end-code.first_start > nn {
				group.push( Opcode::new(code.tag.clone(), code.first_start, min(code.first_end, code.first_start+n),
					code.second_start, min(code.second_end, code.second_start+n)) );
				res.push(group.clone());
				group.clear();
				first_start = max(first_start, code.first_end-n);
				second_start = max(second_start, code.second_end - n);
			}
			group.push( Opcode::new(code.tag.clone(), first_start, code.first_end, second_start, code.second_end) );
		}
		if !group.is_empty() && !(group.len()==1 && group.first().unwrap().tag == "equal") {
			res.push(group.clone());
		}
		res
	}

	pub fn ratio(&mut self) -> f32{
		let matches = self.get_matching_blocks().iter().fold(0, |res, &m| res + m.size);
		calculate_ratio(matches, self.first_sequence.len() + self.second_sequence.len())
	}
}
