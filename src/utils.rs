pub fn slice_str(string: &str, start: usize, end: usize) -> Option<&str>
{
	if start >= end{
	    return None
    }
    if start > string.len() || end > string.len(){
    	return None
    }
    unsafe{
    	Some(string.slice_unchecked(start, end))
    }
}


pub fn calculate_ratio(matches: usize, length: usize) -> f32{
	if length != 0{
		return 2.0 * matches as f32 / length as f32
	}
	return 1.0
}


pub fn str_with_similar_chars(c: char, length: usize) -> String {
    let mut s = String::new();
    for _ in 0..length{
        s.push_str(&c.to_string());
    }
    s
}
