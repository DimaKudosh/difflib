use std::cmp::Ordering;
use utils::slice_str;

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


pub struct SequenceMatcher<'a>{
	first_sequence: &'a str,
	second_sequence: &'a str,
	matching_blocks: Option<Vec<Match>>
}

impl<'a> SequenceMatcher<'a>{
	pub fn new(first_sequence: &'a str, second_sequence: &'a str) -> SequenceMatcher<'a>
	{
		SequenceMatcher{
			first_sequence: first_sequence,
			second_sequence: second_sequence,
			matching_blocks: None
		}
	}

	pub fn find_longest_match(&self, first_start: usize, first_end: usize, second_start: usize, second_end: usize) -> Option<Match>
	{ 
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

	pub fn get_matching_blocks(&mut self) -> Vec<Match>
	{
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
		matches.sort_by(|a, b| a.cmp(b));
		let (mut first_start, mut second_start, mut size) = (0, 0, 0);
		let mut non_adjacent = Vec::new();
		for m in &matches{
			if first_start + size == m.first_start && second_start + size == m.second_start{
				size += m.size
			}
			else {
			    if size != 0{
			    	non_adjacent.push(m.clone());
			    }
			    first_start = m.first_start;
			    second_start = m.second_start;
			    size = m.size;
			}
		}	
		self.matching_blocks = Some(non_adjacent);
		self.matching_blocks.as_ref().unwrap().clone()
	}
}