use crate::utils::io::{read_line, read_value};
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29731(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let promises: HashSet<&str> = HashSet::from([
        "Never gonna give you up",
        "Never gonna let you down",
        "Never gonna run around and desert you",
        "Never gonna make you cry",
        "Never gonna say goodbye",
        "Never gonna tell a lie and hurt you",
        "Never gonna stop",
    ]);

    let ans = (0..n)
        .map(|_| read_line(reader))
        .all(|s| promises.contains(s.as_str()));

    write!(writer, "{}", if ans { "No" } else { "Yes" }).unwrap();
}

// https://www.acmicpc.net/problem/29731
// noinspection SpellCheckingInspection
// 2033년 밈 투표
#[test]
fn test_solve29731() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
Never gonna give you up
Never gonna say goodbye
Let you down"
                .to_string(),
            want: "Yes".to_string(),
        },
        TestData {
            s: "7
Never gonna give you up
Never gonna let you down
Never gonna run around and desert you
Never gonna make you cry
Never gonna say goodbye
Never gonna tell a lie and hurt you
Never gonna stop"
                .to_string(),
            want: "No".to_string(),
        },
        TestData {
            s: "7
Never gonna give you up
Never gonna let you down
Never gonna run around and desert you
Never gonna make you cry
Never gonna say good bye
Never gonna tell a lie and hurt you
Never gonna stop"
                .to_string(),
            want: "Yes".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve29731(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
