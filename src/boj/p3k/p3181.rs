use crate::utils::io::read_line;
use std::collections::HashSet;
use std::io::{BufRead, Write};

// noinspection SpellCheckingInspection
#[allow(dead_code)]
fn solve3181(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let words = s.split_whitespace().collect::<Vec<_>>();
    let prepositions = [
        "i", "pa", "te", "ni", "niti", "a", "ali", "nego", "no", "ili",
    ]
    .iter()
    .cloned()
    .collect::<HashSet<_>>();
    let mut ans = words[0].chars().next().unwrap().to_string().to_uppercase();

    for word in words.iter().skip(1) {
        if !prepositions.contains(word) {
            ans.push(word.chars().next().unwrap().to_uppercase().next().unwrap());
        }
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/3181
// noinspection SpellCheckingInspection
// 줄임말
#[test]
fn test_solve3181() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "micro soft".to_string(),
            want: "MS".to_string(),
        },
        TestData {
            s: "biti ali i ne biti".to_string(),
            want: "BNB".to_string(),
        },
        TestData {
            s: "ali ja sam i jucer jeo".to_string(),
            want: "AJSJJ".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3181(&mut reader, &mut writer);

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
