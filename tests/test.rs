extern crate difflib;

use difflib::sequencematcher::{SequenceMatcher, Match, Opcode};
use difflib::differ::Differ;

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

#[test]
fn test_get_opcodes() {
    let mut matcher = SequenceMatcher::new("qabxcd", "abycdf");
    let result = matcher.get_opcodes();
    let mut expected_result = Vec::new();
    expected_result.push(Opcode{
        tag: "delete".to_string(),
        first_start: 0,
        first_end: 1,
        second_start: 0,
        second_end: 0
    });
    expected_result.push(Opcode{
        tag: "equal".to_string(),
        first_start: 1,
        first_end: 3,
        second_start: 0,
        second_end: 2
    });
    expected_result.push(Opcode{
        tag: "replace".to_string(),
        first_start: 3,
        first_end: 4,
        second_start: 2,
        second_end: 3
    });
    expected_result.push(Opcode{
        tag: "equal".to_string(),
        first_start: 4,
        first_end: 6,
        second_start: 3,
        second_end: 5
    });
    expected_result.push(Opcode{
        tag: "insert".to_string(),
        first_start: 6,
        first_end: 6,
        second_start: 5,
        second_end: 6
    });
    assert_eq!(result, expected_result);
}

#[test]
fn test_ratio() {
    let mut matcher = SequenceMatcher::new("abcd", "bcde");
    assert_eq!(matcher.ratio(), 0.75);
}

#[test]
fn test_get_close_matches() {
    let words = vec!["ape", "apple", "peach", "puppy"];
    let result = difflib::get_close_matches("appel", words, 3, 0.6);
    assert_eq!(result, vec!["apple", "ape"]);
}

#[test]
fn test_differ_compare() {
    let first_text = vec!["one\n", "two\n", "three\n"];
    let second_text = vec!["ore\n", "tree\n", "emu\n"];
    let differ = Differ::new();
    let result = differ.compare(&first_text, &second_text).join("");
    assert_eq!(result, "- one\n?  ^\n+ ore\n?  ^\n- two\n- three\n?  -\n+ tree\n+ emu\n");
}