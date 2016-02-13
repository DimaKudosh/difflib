#[derive(Debug)]
pub struct Match{
	first_start: usize,
	second_start: usize,
	size: usize
}

pub struct SequenceMatcher<'a>{
	first_sequence: &'a str,
	second_sequence: &'a str
}

impl<'a> SequenceMatcher<'a>{
	pub fn new(first_sequence: &'a str, second_sequence: &'a str) -> SequenceMatcher<'a>
	{
		SequenceMatcher{
			first_sequence: first_sequence,
			second_sequence: second_sequence
		}
	}

	pub fn find_longest_match(&self, first_start: usize, first_end: usize, second_start: usize, second_end: usize) -> Match
	{
		let first_sequence: Vec<char> = self.first_sequence.chars().collect();
		let second_sequence: Vec<char> = self.second_sequence.chars().collect();
		let mut start = 0;
		let mut end = 0;
		let mut arr: Vec<Vec<usize>> = vec![vec![0; second_sequence.len() + 1]; first_sequence.len() + 1];
		for i in 0..first_sequence.len(){
			for j in 0..second_sequence.len(){
				if first_sequence[i] == second_sequence[j]{
					arr[i + 1][j + 1] = arr[i][j] + 1;
					if arr[i + 1][j + 1] > arr[start][end]{
						start = i + 1;
						end = j + 1;
					}
				}
			}
		}
		Match{
			first_start: start - arr[start][end],
			second_start: end - arr[start][end],
			size: arr[start][end]
		}
	}

	pub fn get_matching_blocks(&self) //-> Vec<Match>
	{
		let first_length = self.first_sequence.len();
		let second_length = self.second_sequence.len();
	}
}