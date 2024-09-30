use std::convert::{TryFrom, TryInto};

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

pub fn char_to_index<T>(c: char) -> T
where
    T: TryFrom<u64> + Copy,
{
    let sub = match c {
        'a'..='z' => b'a',
        'A'..='Z' => b'A',
        '0'..='9' => b'0',
        _ => panic!("Character is not a valid ASCII letter"),
    };

    let value = u64::from(c)
        .checked_sub(u64::from(sub))
        .expect("Subtraction underflow");
    T::try_from(value).ok().expect("Conversion failed")
}

pub fn char_to_digit<T: TryFrom<u64> + Copy>(c: char) -> T {
    u64::from(c).try_into().ok().expect("Conversion failed")
}
