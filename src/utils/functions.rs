pub fn check_palindrome_nth(s: &[u8], i: usize) -> bool {
    s[i] == s[s.len() - 1 - i]
}
