extern crate regex;
extern crate ascii;
use std::string::String;
use std::io::Read;
use std::i32;
use std::ops::Add;
use super::parse_utils;

pub fn day_9(){
    let inp = parse_utils::parse_input_file("inp9.txt");
    let stripped: String = remove_ignored_characters(&inp);
    let removed_garbage = count_all_garbage(stripped);
    println!("Reslut 9 b = {}",removed_garbage );

    let stripped2: String = remove_ignored_characters(&inp);

    let all_garbage_removed = remove_all_garbage(stripped2);
    let find_groups = find_groups(all_garbage_removed);
   println!("Reslut 9 a = {}",find_groups );
}



pub fn find_groups(text: String) -> i32
{
    let mut group_found = 0;
    let mut left = 0;
    for ch in text.chars() {
        match ch {
            '{' => left += 1,
            '}' => {
                if left > 0 {
                    group_found += left;
                    left -= 1;
                }
            },
            _ => ()// Do nothing.
        }
    }

    return  group_found;

}
pub fn remove_ignored_characters(text: &str) -> String
{
    let mut stripped = String::new();
    let mut previous = 'c';
    let mut previous_is_cancelled = false;
    for s in text.chars(){
        if previous == '!' && !previous_is_cancelled {
            previous_is_cancelled = true;
        } else {
            if s != '!' {
                stripped = stripped.add(&s.to_string());
            }
            previous_is_cancelled = false;
        }
        previous = s;
    }
    return stripped;
}


trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
    fn remove_substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
    fn remove_substring(&self, start: usize, end: usize) -> Self {
        let mut start: Vec<char> = self.chars().take(start).collect();
        let end: Vec<char> = self.chars().skip(end).collect();
        start.extend(&end);
        return start.into_iter().collect()
    }
}

pub fn remove_all_garbage(text: String) -> String
{
    let mut updated_text: String = text;

    let grabage_removed: String =  loop {
        let copy = updated_text.clone();
        let new_updated_text = remove_included_garbage(copy);
        if new_updated_text == updated_text {
            return updated_text;
        }
        updated_text = new_updated_text;
    };

    return  grabage_removed;
}


pub fn count_all_garbage(text: String) -> usize
{
    let mut updated_text: String = text;
    let mut count : usize = 0;
    loop {
        let copy = updated_text.clone();
        let count_this = &copy.len();
        let new_updated_text = remove_included_garbage(copy);
        if new_updated_text == updated_text {
            return count;
        }
        let count_new = new_updated_text.len();
        count = count + count_this - count_new - 2;
        updated_text = new_updated_text;
    };

    return  count;
}

pub fn remove_included_garbage(text: String) -> String
{
    let mut remove_starts_at: Option<usize> = None;
    let mut remove_stops_at: Option<usize> = None;

    let mut iter = 0;
    for s in text.chars(){

        if s == '<' && remove_starts_at.is_none() {
            remove_starts_at = Some(iter);
        }
        else if s == '>' && remove_starts_at.is_some() {
            remove_stops_at = Some(iter + 1);
            break;
        }
        iter = iter + 1;
    }
    if remove_starts_at.is_none() || remove_stops_at.is_none() {
        return text;
    }
    return text.remove_substring(remove_starts_at.unwrap(), remove_stops_at.unwrap());
}


#[cfg(test)]
mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_9();    // 4322 and 9251
    }
    #[test]
    pub fn test_day_9()
    {
        let stripped: String = super::remove_ignored_characters("{{<!>},{<!>},{<!>},{<a>}}");
        assert_eq!(stripped, "{{<},{<},{<},{<a>}}");

        let stripped: String = super::remove_ignored_characters("{{<!!>}");
        assert_eq!(stripped, "{{<>}");
        let stripped: String = super::remove_ignored_characters("{{<!!!>}");
        assert_eq!(stripped, "{{<}");

        let removed_garbage: String = super::remove_included_garbage("<random characters>".to_string());
        assert_eq!(removed_garbage, "");

        let removed_garbage: String = super::remove_included_garbage("<ran>dom characters>".to_string());
        assert_eq!(removed_garbage, "dom characters>");

        let removed_garbage: String = super::remove_included_garbage("{{<!},{<!},{<!},{<a>}}".to_string());
        assert_eq!(removed_garbage, "{{}}");

        let removed_garbage: String = super::remove_all_garbage("gfag<{oi!a,<{i<a>ghjkghg<{oi!a,<{i<a><{oi!a>,<{i<a>j".to_string());
        assert_eq!(removed_garbage, "gfagghjkghg,j");

        let find_groups = super::find_groups("{{{},{},{{}}}}".to_string());
        assert_eq!(find_groups, 16);
        let find_groups = super::find_groups("{{},{}}".to_string());
        assert_eq!(find_groups, 5);
        let find_groups = super::find_groups("{{{}}}".to_string());
        assert_eq!(find_groups, 6);

        let removed_garbage = super::remove_included_garbage("{<{},{},{{}}>}".to_string());
        let find_groups = super::find_groups(removed_garbage);
        assert_eq!(find_groups, 1);
        let removed_garbage = super::remove_included_garbage("{<a>,<a>,<a>,<a>}".to_string());
        let find_groups = super::find_groups(removed_garbage);
        assert_eq!(find_groups, 1);
        let removed_garbage = super::remove_included_garbage("{{<a>},{<a>},{<a>},{<a>}}".to_string());
        let find_groups = super::find_groups(removed_garbage);
        assert_eq!(find_groups, 9);

        let stripped: String = super::remove_ignored_characters("{{<!>},{<!>},{<!>},{<a>}}");
        let removed_garbage = super::remove_included_garbage(stripped);
        let find_groups = super::find_groups(removed_garbage);
        assert_eq!(find_groups, 3);
    }
}

