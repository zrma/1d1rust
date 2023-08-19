pub fn check_palindrome_nth(s: &Vec<char>, i: usize) -> bool {
    s[i] == s[s.len() - 1 - i]
}
