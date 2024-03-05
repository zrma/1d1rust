use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1718(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = reader.lines().next().unwrap().unwrap();
    let key = reader.lines().next().unwrap().unwrap();

    let mut res = String::new();
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            res.push(' ');
            continue;
        }

        let key_char = key.as_bytes()[i % key.len()] as char;
        let key_num = key_char as i8 - 'a' as i8 + 1;
        let c_num = c as i8 - 'a' as i8 + 1;

        let mut res_num = c_num - key_num;
        if res_num <= 0 {
            res_num += 26;
        }

        res.push((res_num as u8 + b'a' - 1) as char);
    }

    write!(writer, "{}", res).expect("Failed to write");
}

// https://www.acmicpc.net/problem/1718
// 암호
// noinspection SpellCheckingInspection
#[test]
fn test_solve1718() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "nice day
love"
                .to_string(),
            want: "btgz oet".to_string(),
        },
        TestData {
            s: "          \n ".to_string(),
            want: "          ".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1718(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
