use crate::utils::io::{read_line, read_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5363(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();

    for _ in 0..n {
        let mut words = read_values::<String>(reader);

        rotate_first_two_words(&mut words);

        writeln!(writer, "{}", words.join(" ")).unwrap();
    }
}

fn rotate_first_two_words(words: &mut Vec<String>) {
    let first_two = words.drain(..2).collect::<Vec<_>>();
    words.extend(first_two);
}

// https://www.acmicpc.net/problem/5363
// 요다
#[test]
fn test_solve5363() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
I will go now to find the Wookiee
Solo found the death star near planet Kessel
I'll fight Darth Maul here and now
Vader will find Luke before he can escape"
                .to_string(),
            want: "go now to find the Wookiee I will
the death star near planet Kessel Solo found
Darth Maul here and now I'll fight
find Luke before he can escape Vader will
"
            .to_string(),
        },
        TestData {
            s: "1
I will go now to find the Wookiee"
                .to_string(),
            want: "go now to find the Wookiee I will
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5363(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
