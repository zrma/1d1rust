use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4892(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut answers = vec![];
    let mut case_num = 1;
    loop {
        let n0: i32 = read_value(read_line(reader));
        if n0 == 0 {
            break;
        }

        let n1 = 3 * n0;
        let n2 = if n1 % 2 == 0 { n1 / 2 } else { (n1 + 1) / 2 };
        let n3 = 3 * n2;
        let n4 = n3 / 9;

        answers.push(format!(
            "{}. {} {}",
            case_num,
            if n0 % 2 == 0 { "even" } else { "odd" },
            n4
        ));
        case_num += 1;
    }

    write!(writer, "{}", answers.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/4892
// 숫자 맞추기 게임
#[test]
fn test_solve4892() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "37
38
0"
            .to_string(),
            want: "1. odd 18
2. even 19"
                .to_string(),
        },
        TestData {
            s: "1
0"
            .to_string(),
            want: "1. odd 0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4892(&mut reader, &mut writer);

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
