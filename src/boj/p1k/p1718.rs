use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1718(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = reader.lines().next().unwrap().unwrap();
    let key = reader.lines().next().unwrap().unwrap();
    let keys_as_bytes = key.as_bytes();

    let mut answers = String::new();
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            answers.push(' ');
            continue;
        }

        let key_char = char::from(keys_as_bytes[i % key.len()]);
        let key_num = key_char
            .to_digit(36)
            .expect("Failed to convert char to digit")
            - 10;
        let c_num = c.to_digit(36).expect("Failed to convert char to digit") - 10;

        let ans = if c_num <= key_num {
            26 + c_num - key_num
        } else {
            c_num - key_num
        };
        answers.push(char::from_u32(ans + 96).expect("Failed to convert u32 to char"));
    }

    write!(writer, "{}", answers).expect("Failed to write");
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
