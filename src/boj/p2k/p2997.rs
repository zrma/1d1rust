use crate::utils::io::read_n_values;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2997(reader: &mut impl BufRead, writer: &mut impl Write) {
    let arr = {
        let mut temp_arr: Vec<i32> = read_n_values(reader, 3);
        temp_arr.sort_unstable();
        temp_arr
    };

    let res = {
        let diff1 = arr[1] - arr[0];
        let diff2 = arr[2] - arr[1];
        match diff1.cmp(&diff2) {
            Equal => (arr[2] + diff1).to_string(),
            Greater => (arr[0] + diff2).to_string(),
            Less => (arr[1] + diff1).to_string(),
        }
    };

    writeln!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/2997
// 네 번째 수
#[test]
fn test_solve2997() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 6 8".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "4 6 7".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "10 1 4".to_string(),
            want: "7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2997(&mut reader, &mut writer);

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
