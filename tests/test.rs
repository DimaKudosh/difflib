extern crate difflib;

use difflib::sequencematcher::{SequenceMatcher, Match};


#[test]
fn test_longest_match()
{
	let matcher = SequenceMatcher::new(" abcd", "abcd abcd");
    let m =  matcher.find_longest_match(0, 5, 0, 9);
    assert_eq!(m.first_start, 0);
    assert_eq!(m.second_start, 4);
    assert_eq!(m.size, 5);
}

#[test]
fn test_all_matches()
{
	let mut matcher = SequenceMatcher::new("abxcd", "abcd");
	let result = matcher.get_matching_blocks();
    let mut expected_result = Vec::new();
    expected_result.push(Match{
    	first_start: 0,
    	second_start: 0,
    	size: 2
    });
    expected_result.push(Match{
    	first_start: 3,
    	second_start: 2,
    	size: 2
    });
    expected_result.push(Match{
    	first_start: 5,
    	second_start: 4,
    	size: 0
    });
    assert_eq!(result, expected_result);
}