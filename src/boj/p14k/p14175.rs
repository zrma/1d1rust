use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14175(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m, k) = read_values_as!(read_line(reader), usize, usize, usize);

    let mut board = Vec::with_capacity(n * k);
    for _ in 0..n {
        let s = read_line(reader);
        let s: String = s
            .chars()
            .take(m)
            .flat_map(|c| std::iter::repeat(c).take(k))
            .collect::<_>();
        for _ in 0..k {
            board.push(s.clone());
        }
    }

    let ans = board.join("\n");
    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14175
// noinspection SpellCheckingInspection
// The Cow-Signal
#[test]
fn test_solve14175() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 4 2
XXX.
X..X
XXX.
X..X
XXX."
                .to_string(),
            want: "XXXXXX..
XXXXXX..
XX....XX
XX....XX
XXXXXX..
XXXXXX..
XX....XX
XX....XX
XXXXXX..
XXXXXX.." // 10
                .to_string(),
        },
        TestData {
            s: "1 1 1
X"
            .to_string(),
            want: "X".to_string(),
        },
        TestData {
            s: "2 2 3
X.
.X"
            .to_string(),
            want: "XXX...
XXX...
XXX...
...XXX
...XXX
...XXX"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve14175(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
