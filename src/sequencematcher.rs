use std::cmp::{Ordering, max, min};
use utils::{slice_str, calculate_ratio};

#[derive(Debug, Clone, Copy)]
pub struct Match{
	first_start: usize,
	second_start: usize,
	size: usize
}

impl Ord for Match {
    fn cmp(&self, other: &Match) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Match) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Match) -> bool {
        self.size == other.size
    }
}

impl Eq for Match { }

impl Match{
	fn new(first_start: usize, second_start: usize, size: usize) -> Match {
		Match{
			first_start: first_start, 
			second_start: second_start,
			size: size
		}
	}
}

#[derive(Debug, Clone)]
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


pub struct SequenceMatcher<'a>{
	first_sequence: &'a str,
	second_sequence: &'a str,
	matching_blocks: Option<Vec<Match>>,
	opcodes: Option<Vec<Opcode>>,
	autojunk: bool
}


impl<'a> SequenceMatcher<'a>{
	pub fn new(first_sequence: &'a str, second_sequence: &'a str) -> SequenceMatcher<'a> {
		SequenceMatcher{
			first_sequence: first_sequence,
			second_sequence: second_sequence,
			matching_blocks: None,
			opcodes: None,
			autojunk: true,

		}
	}

	pub fn set_seqs(&mut self, first_sequence: &'a str, second_sequence: &'a str) {
		self.set_first_seq(first_sequence);
		self.set_second_seq(second_sequence);
	}

	pub fn set_first_seq(&mut self, sequence: &'a str) {
		self.first_sequence = sequence;
		self.matching_blocks = None;
		self.opcodes = None;
	}

	pub fn set_second_seq(&mut self, sequence: &'a str) {
		self.second_sequence = sequence;
		self.matching_blocks = None;
		self.opcodes = None;
		//self.fullbcount = None
        //self.__chain_b()
	}

	pub fn find_longest_match(&self, first_start: usize, first_end: usize, second_start: usize, second_end: usize) -> Option<Match> { 
		let first_sequence: Vec<char> = slice_str(self.first_sequence, first_start, first_end).unwrap_or("").chars().collect();
		let second_sequence: Vec<char> = slice_str(self.second_sequence, second_start, second_end).unwrap_or("").chars().collect();
		let mut max_i = 0;
		let mut max_j = 0;
		let mut arr: Vec<Vec<usize>> = vec![vec![0; second_sequence.len() + 1]; first_sequence.len() + 1];
		for i in 0..first_sequence.len(){
			for j in 0..second_sequence.len(){
				if first_sequence[i] == second_sequence[j]{
					arr[i + 1][j + 1] = arr[i][j] + 1;
					if arr[i + 1][j + 1] > arr[max_i][max_j] {
                        max_i = i + 1;
                        max_j = j + 1;
                   }
				}
			}
		}
		if max_i == 0 && max_j == 0{
			return None
		}
		else{
			let size = arr[max_i][max_j];
			Some(Match{
				first_start: max_i - size + first_start,
				second_start: max_j - size + second_start,
				size: size
			})
		}
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
			match m {
			    Some(m) => {
			    	if first_start < m.first_start && second_start < m.second_start{
			    		queue.push((first_start, m.first_start, second_start, m.second_start));
			    	}
			    	if m.first_start + m.size < first_end && m.second_start + m.size < second_end{
			    		queue.push((m.first_start + m.size, first_end, m.second_start + m.size, second_end));
			    	}
			    	matches.push(m);
			    },
			    None => {},
			}
		}
		matches.sort_by(|a, b| b.cmp(a));
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

	/*
	codes = self.get_opcodes()
        if not codes:
            codes = [("equal", 0, 1, 0, 1)]
        # Fixup leading and trailing groups if they show no changes.
        if codes[0][0] == 'equal':
            tag, i1, i2, j1, j2 = codes[0]
            codes[0] = tag, max(i1, i2-n), i2, max(j1, j2-n), j2
        if codes[-1][0] == 'equal':
            tag, i1, i2, j1, j2 = codes[-1]
            codes[-1] = tag, i1, min(i2, i1+n), j1, min(j2, j1+n)

        nn = n + n
        group = []
        for tag, i1, i2, j1, j2 in codes:
            # End the current group and start a new one whenever
            # there is a large range with no changes.
            if tag == 'equal' and i2-i1 > nn:
                group.append((tag, i1, min(i2, i1+n), j1, min(j2, j1+n)))
                yield group
                group = []
                i1, j1 = max(i1, i2-n), max(j1, j2-n)
            group.append((tag, i1, i2, j1 ,j2))
        if group and not (len(group)==1 and group[0][0] == 'equal'):
            yield group
           */
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
