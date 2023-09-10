use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9946(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut case = 0;
    loop {
        case += 1;
        let line1 = read_line(reader);
        let line2 = read_line(reader);

        if line1 == "END" && line2 == "END" {
            break;
        }

        let chars1 = line1.chars().fold([0; 26], |mut acc, c| {
            acc[c as usize - 'a' as usize] += 1;
            acc
        });

        let chars2 = line2.chars().fold([0; 26], |mut acc, c| {
            acc[c as usize - 'a' as usize] += 1;
            acc
        });

        let is_same = chars1.iter().zip(chars2.iter()).all(|(a, b)| a == b);
        if is_same {
            writeln!(writer, "Case {}: same", case).unwrap();
        } else {
            writeln!(writer, "Case {}: different", case).unwrap();
        }
    }
}

// https://www.acmicpc.net/problem/9946
// 단어 퍼즐
// noinspection SpellCheckingInspection
#[test]
fn test_solve9946() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "testing
intestg
abc
aabbbcccc
abcabcbcc
aabbbcccc
abc
xyz
END
END"
            .to_string(),
            want: "Case 1: same
Case 2: different
Case 3: same
Case 4: different
"
            .to_string(),
        },
        TestData {
            s: "testing
intestg
END
END"
            .to_string(),
            want: "Case 1: same
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9946(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {} failed", i);
    }
}
