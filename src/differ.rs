use sequencematcher::{SequenceMatcher, Sequence};
use utils::{str_with_similar_chars, count_leading};
use std::cmp;


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

	pub fn compare<T: ?Sized + Sequence>(&self, first_sequence: &T, second_sequence: &T) -> Vec<Vec<String>> {
		let mut matcher = SequenceMatcher::new(first_sequence, second_sequence);
		let mut res = Vec::new();
		for opcode in matcher.get_opcodes(){
			match opcode.tag.as_ref() {
			    "replace" => { res.push( self.fancy_replace(first_sequence, opcode.first_start, opcode.first_end, 
			    	second_sequence, opcode.second_start, opcode.second_end) ) },
			    "delete" => { res.push( self.dump("-", first_sequence, opcode.first_start, opcode.first_end) ) },
			    "insert" => { res.push( self.dump("+", second_sequence, opcode.second_start, opcode.second_end) ) },
			    "equal" => { res.push( self.dump(" ", first_sequence, opcode.first_start, opcode.first_end) ) },
			    _ => {}
			}
		}
		res
	}

	fn dump<T: ?Sized + Sequence>(&self, tag: &str, sequence: &T, start: usize, end: usize) -> Vec<String> {
		let mut res = Vec::new();
        for i in start..end {
            match sequence.at_index(i) {
                Some(s) => res.push(format!("{} {}", tag, s)),
                None => {},
            }
        } 
		res
	}

    fn plain_replace<T: ?Sized + Sequence>(&self, first_sequence: &T, first_start: usize, first_end: usize, 
    	second_sequence: &T, second_start: usize, second_end: usize) -> Vec<String> {
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
    	for str in second{
    		first.push(str);
    	}
    	first
    }

    fn fancy_replace<T: ?Sized + Sequence>(&self, first_sequence: &T, first_start: usize, first_end: usize,
    	second_sequence: &T, second_start: usize, second_end: usize) -> Vec<String> {
    	let mut res = Vec::new();
    	let (mut best_ratio, cutoff) = (0.74, 0.75);
    	let (mut best_i, mut best_j) = (0, 0);
    	let (mut second_sequence_str, mut first_sequence_str) = ("", "");
    	//cruncher.charjunk = self.charjunk;
    	let mut eqi: Option<usize> = None;
    	let mut eqj: Option<usize> = None;
    	for j in second_start..second_end{
    		second_sequence_str = second_sequence.at_index(j).unwrap();  // Fix in feature
    		for i in first_start..first_end{
    			first_sequence_str = first_sequence.at_index(i).unwrap();
    			if first_sequence_str == second_sequence_str{ 
    				if eqi.is_none(){
    					eqi = Some(i);
    					eqj = Some(j);
    				}
    				continue;
    			}
    			let mut cruncher = SequenceMatcher::new(first_sequence_str, second_sequence_str);
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
        	    return res
        	}
        	best_i = eqi.unwrap();
        	best_j = eqj.unwrap();
        	best_ratio = 1.0;
        } else {
        	eqi = None;
        }
        res.extend(self.fancy_helper(first_sequence, first_start, best_i, second_sequence, second_start, best_j).iter().cloned());
        let (first_elt, second_elt) = (first_sequence_str.at_index(best_i).unwrap(), second_sequence_str.at_index(best_j).unwrap());
        if eqi.is_none(){
        	let (mut first_tag, mut second_tag) = (String::new(), String::new());
        	let mut cruncher = SequenceMatcher::new(first_elt, second_elt);
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
        		res.extend(self.qformat(&first_elt, &second_elt, &first_tag, &second_tag).iter().cloned());
        	}
        } else {
        	let mut s = String::from("  ");
        	s.push_str(&first_elt);
        	res.extend(vec![s].iter().cloned());
        }
        res.extend(self.fancy_helper(first_sequence, best_i + 1, first_end, second_sequence, best_j + 1, second_end).iter().cloned());
        res
    }

    fn fancy_helper<T: ?Sized + Sequence>(&self, first_sequence: &T, first_start: usize, first_end: usize,
    	second_sequence: &T, second_start: usize, second_end: usize) -> Vec<String> {
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

    fn qformat(&self, first_line: &str, second_line: &str, first_tags: &str, second_tags: &str) -> Vec<String> {
    	let mut res = Vec::new();
    	let mut common = cmp::min(count_leading(first_line, '\t'), count_leading(second_line, '\t'));
    	common = cmp::min(common, count_leading(first_tags.split_at(common).0, ' '));
    	common = cmp::min(common, count_leading(first_tags.split_at(common).0, ' '));
        let mut s = String::from(format!("- {}", first_line));
        res.push(s);
        if first_tags != ""{
        	s = String::from(format!("? {}{}\n", str_with_similar_chars('\t', common), first_tags));
        	res.push(s);
        }
        s = String::from(format!("+ {}", second_line));
        res.push(s);
        if second_tags != "" {
        	s = String::from(format!("? {}{}\n", str_with_similar_chars('\t', common), second_tags));
        	res.push(s);
        }
        res
    }

}
