use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve26264(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_of_words: usize = read_value(read_line(reader));
    let input = read_line(reader);
    let input = input.as_bytes();

    let mut cnt_of_bigdata = 0;
    let mut cnt_of_security = 0;

    let mut current_index = 0;
    while current_index < input.len() && cnt_of_bigdata + cnt_of_security < num_of_words {
        if input[current_index] == b'b' {
            cnt_of_bigdata += 1;
            current_index += "bigdata".len();
        } else {
            cnt_of_security += 1;
            current_index += "security".len();
        }
    }

    let ans = match cnt_of_bigdata.cmp(&cnt_of_security) {
        Less => "security!",
        Equal => "bigdata? security!",
        Greater => "bigdata?",
    };

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/26264
// noinspection SpellCheckingInspection
// 빅데이터? 정보보호!
#[test]
fn test_solve26264() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
securitybigdatasecuritybigdatasecurity"
                .to_string(),
            want: "security!".to_string(),
        },
        TestData {
            s: "1
bigdata"
                .to_string(),
            want: "bigdata?".to_string(),
        },
        TestData {
            s: "6
bigdatabigdatabigdatasecuritysecuritysecurity"
                .to_string(),
            want: "bigdata? security!".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve26264(&mut reader, &mut writer);

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
