use sequencematcher::SequenceMatcher;
use utils::{slice_str, str_with_similar_chars};


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

	pub fn compare(&self, first_sequence: &str, second_sequence: &str) -> Vec<Vec<String>> {
		let mut matcher = SequenceMatcher::new(first_sequence, second_sequence);
		let mut res = Vec::new();
		for opcode in matcher.get_opcodes(){
			match opcode.tag.as_ref() {
			    "replace" => {},
			    "delete" => { res.push( self.dump("-", first_sequence, opcode.first_start, opcode.first_end) ) },
			    "insert" => { res.push( self.dump("+", second_sequence, opcode.second_start, opcode.second_end) ) },
			    "equal" => { res.push( self.dump(" ", first_sequence, opcode.first_start, opcode.first_end) ) },
			    _ => {}
			}
		}
		res
	}

	fn dump(&self, tag: &str, string: &str, start: usize, end: usize) -> Vec<String> {
		let mut res = Vec::new();
		match slice_str(string, start, end) {
		    Some(s) => {
		    	for c in s.chars(){
		    		res.push(format!("{} {}", tag, c));
		    	}
		    },
		    None => {},
		}
		res
	}

    fn plain_replace(&self, first_sequence: &str, first_start: usize, first_end: usize, 
    	second_sequence: &str, second_start: usize, second_end: usize) -> Vec<Vec<String>> {
    	if !(first_start < first_end && second_start < second_end){
    		return Vec::new();
    	}
    	let mut first = Vec::new();
    	let mut second = Vec::new();
    	if second_end - second_start < first_end - first_start{
    		first = self.dump("+", second_sequence, second_start, second_end);
    		second = self.dump("-", first_sequence, first_start, first_end);
    	} else {
    		first = self.dump("-", first_sequence, first_start, first_end);
    		second = self.dump("+", second_sequence, second_start, second_end);
    	}
    	vec![first, second]
    }
    
    fn fancy_replace(&self, first_sequence: &str, first_start: usize, first_end: usize,
    	second_sequence: &str, second_start: usize, second_end: usize) -> Vec<Vec<String>> {
    	let mut res = Vec::new();
    	let first_sequence_vec: Vec<char> = first_sequence.chars().collect();
		let second_sequence_vec: Vec<char> = second_sequence.chars().collect();
    	let (mut best_ratio, cutoff) = (0.74, 0.75);
    	let (mut best_i, mut best_j) = (0, 0);
    	let (mut second_sequence_char, mut first_sequence_char) = (String::new(), String::new());
    	let mut cruncher = SequenceMatcher::new("", "");
    	//cruncher.charjunk = self.charjunk;
    	// eqi, eqj = None, None   # 1st indices of equal lines (if any)
    	let mut eqi: Option<usize> = None;
    	let mut eqj: Option<usize> = None;
    	for j in second_start..second_end{
    		cruncher.set_first_seq("");
    		cruncher.set_second_seq("");
    		second_sequence_char = second_sequence_vec[j].to_string();
    	    //cruncher.set_second_seq(&second_sequence_char);
    		for i in first_start..first_end{
    			first_sequence_char = first_sequence_vec[i].to_string();
    			if first_sequence_char == second_sequence_char{
    				if eqi.is_none(){
    					eqi = Some(i);
    					eqj = Some(j);
    				}
    				continue;
    			}
    			//cruncher.set_first_seq(&first_sequence_char);
    			if cruncher.ratio() > best_ratio{
    				best_ratio = cruncher.ratio();
    				best_i = i;
    				best_j = j;
    			}
    		}
        }
        if best_ratio < cutoff{
        	if eqi.is_none(){
        		res.extend(self.plain_replace(first_sequence, first_start, first_end, second_sequence, second_start, second_end).iter().cloned());
        	}
        	best_i = eqi.unwrap();
        	best_j = eqj.unwrap();
        	best_ratio = 1.0;
        } else {
        	eqi = None;
        }
        res.extend(vec![self.fancy_helper(first_sequence, first_start, best_i, second_sequence, second_start, best_j)].iter().cloned());
        let (first_elt, second_elt) = (first_sequence_vec[best_i].to_string(), second_sequence_vec[best_j].to_string());
        if eqi.is_none(){
        	let (mut first_tag, mut second_tag) = (String::new(), String::new());
        	//cruncher.set_seqs(&first_elt, &second_elt);
        	for opcode in &cruncher.get_opcodes(){
        		let (first_length, second_length) = (opcode.first_end - opcode.first_start, opcode.second_end - opcode.second_start);
        		match opcode.tag.as_ref() {
        		    "replace" => {
        		    	first_tag.push_str(&str_with_similar_chars('^', first_length));
        			    second_tag.push_str(&str_with_similar_chars('^', second_length));
        		    },
        		    "delete" => {
        		    	first_tag.push_str(&str_with_similar_chars('^', first_length));
        		    },
        		    "insert" => {
        		    	second_tag.push_str(&str_with_similar_chars('^', second_length));
        		    },
        		    "equal" => {
        		    	first_tag.push_str(&str_with_similar_chars(' ', first_length));
        		    	second_tag.push_str(&str_with_similar_chars(' ', second_length));
        		    },
        		    _ => {}
        		}
        		//res.extend(self.qformat)
        	}
        } else {
        	let mut s = String::from("  ");
        	s.push_str(&first_elt);
        	res.extend(vec![vec![s]].iter().cloned());
        }
        res.extend(vec![self.fancy_helper(first_sequence, best_i + 1, first_end, second_sequence, best_j + 1, second_end)].iter().cloned());
        res
    }

    fn fancy_helper(&self, first_sequence: &str, first_start: usize, first_end: usize,
    	second_sequence: &str, second_start: usize, second_end: usize) -> Vec<String> {
    	let mut res = Vec::new();
    	if first_start < first_end{
    		if second_start < second_end{
    			res = self.fancy_replace(first_sequence, first_start, first_end, second_sequence, second_start, second_end);
    		} else {
    			res = self.dump("-", first_sequence, first_start, first_end);
    		}
    	} else if second_start < second_end {
    		res = self.dump("+", second_sequence, second_start, second_end);
    	}
    	res
    }
}