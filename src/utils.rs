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