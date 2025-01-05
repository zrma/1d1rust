use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15819(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (num_words, target_index) = read_values_as!(read_line(reader), usize, usize);
    let mut words = Vec::with_capacity(num_words);

    for _ in 0..num_words {
        let s = read_line(reader);
        words.push(s);
    }

    words.sort();

    writeln!(writer, "{}", words[target_index - 1]).unwrap();
}

// https://www.acmicpc.net/problem/15819
// noinspection SpellCheckingInspection
// 너의 핸들은
#[test]
fn test_solve15819() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 1
acka1357
spectaclehong
mitslll
luke0201"
                .to_string(),
            want: "acka1357".to_string(),
        },
        TestData {
            s: "9 7
tourist
petr
qilip
won0114
hmy3743
jujh97
hjhj97
bio8641
kangjieun9843"
                .to_string(),
            want: "qilip".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve15819(&mut reader, &mut writer);

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
