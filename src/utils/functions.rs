pub fn check_palindrome_nth(s: &[u8], i: usize) -> bool {
    s[i] == s[s.len() - 1 - i]
}

pub fn try_next_pos(
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> Option<(usize, usize)> {
    let nx = x.checked_add_signed(dx)?;
    let ny = y.checked_add_signed(dy)?;
    if nx < w && ny < h {
        Some((nx, ny))
    } else {
        None
    }
}
