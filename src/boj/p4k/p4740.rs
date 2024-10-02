use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4740(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        s = s.trim_matches('\n').to_string();

        if s == "***" {
            break;
        }

        writeln!(writer, "{}", s.chars().rev().collect::<String>()).unwrap();
    }
}

// https://www.acmicpc.net/problem/4740
// 거울, 오! 거울
#[test]
fn test_solve4740() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "Hello, world!
***"
            .to_string(),
            want: "!dlrow ,olleH
"
            .to_string(),
        },
        TestData {
            s: "I am happy today. \n***
"
            .to_string(),
            want: " .yadot yppah ma I
"
            .to_string(),
        },
        TestData {
            s: "AMBULANCE
Evian
madam, i'm adam
***"
            .to_string(),
            want: "ECNALUBMA
naivE
mada m'i ,madam
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4740(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
