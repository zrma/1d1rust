use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5893(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader);

    // n * 17 = n << 4 + n
    let shifted_n = format!("{}0000", n); // n << 4
    let ans = add_binary_iter(&shifted_n, &n);

    write!(writer, "{}", vec_to_string(ans)).expect("write! should work");
}

fn add_binary_iter(a: &str, b: &str) -> impl DoubleEndedIterator<Item = u8> {
    let max_len = a.len().max(b.len());
    let a_iter = a.bytes().rev().chain(std::iter::repeat(b'0')).take(max_len);
    let b_iter = b.bytes().rev().chain(std::iter::repeat(b'0')).take(max_len);

    let (result, carry) = a_iter.zip(b_iter).fold(
        (Vec::with_capacity(max_len + 1), 0),
        |(mut acc, carry), (a_digit, b_digit)| {
            let sum = carry + (a_digit - b'0') + (b_digit - b'0');
            acc.push(sum % 2);
            (acc, sum / 2)
        },
    );

    result.into_iter().chain(if carry > 0 {
        Some(1).into_iter()
    } else {
        None.into_iter()
    })
}

fn vec_to_string<I>(iter: I) -> String
where
    I: Iterator<Item = u8> + DoubleEndedIterator,
{
    iter.rev().map(|bit| char::from(b'0' + bit)).collect()
}

// https://www.acmicpc.net/problem/5893
// 17배
#[test]
fn test_solve5893() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10110111".to_string(),
            want: "110000100111".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "10001".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5893(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
