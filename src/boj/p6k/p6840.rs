use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6840(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut nums = Vec::with_capacity(3);
    for _ in 0..3 {
        let n: i32 = read_line(reader).parse().unwrap();
        nums.push(n);
    }

    nums.sort_unstable();
    writeln!(writer, "{}", nums[1]).unwrap();
}

// https://www.acmicpc.net/problem/6840
// Who is in the middle?
#[test]
fn test_solve6840() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "10
5
8"
            .to_string(),
            want: "8".to_string(),
        },
        TestCase {
            s: "3
1
2"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6840(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
