use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25757(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (total_names, count_type) = read_values_as!(read_line(reader), u32, char);
    let unique_names_set = (0..total_names)
        .map(|_| read_value::<String>(read_line(reader)))
        .collect::<std::collections::HashSet<_>>();

    let divisor = match count_type {
        'Y' => 1,
        'F' => 2,
        _ => 3,
    };

    let answer = unique_names_set.len() / divisor;
    write!(writer, "{}", answer).unwrap();
}

// https://www.acmicpc.net/problem/25757
// noinspection SpellCheckingInspection
// 임스와 함께하는 미니게임
#[test]
fn test_solve25757() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 Y
lms0806
lms0806
exponentiale
lms0806
jthis
lms0806
leo020630"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "12 F
lms0806
powergee
skeep194
lms0806
tony9402
lms0806
wider93
lms0806
mageek2guanaah
lms0806
jthis
lms0806"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "12 O
lms0806
mageek2guanaah
jthis
lms0806
exponentiale
lms0806
leo020630
lms0806
powergee
lms0806
skeep194
lms0806"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25757(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
