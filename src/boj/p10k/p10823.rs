use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10823(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut s = String::new();
    reader.read_to_string(&mut s).unwrap();
    let ans: u64 = s
        .replace('\n', "")
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .sum();
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/10823
// 더하기 2
#[test]
fn test_solve10823() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10,20,
3
0,50
,1
00"
            .to_string(),
            want: "210".to_string(),
        },
        TestData {
            s: "1,2,3,4,5".to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "1,2,3,4,5,\n6,7,8,9,10".to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "1,2,3\n,4,5\n,6,7,8,9,10,11,\n12,\n13,14,15".to_string(),
            want: "120".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10823(&mut reader, &mut writer);

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
