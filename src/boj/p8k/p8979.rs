use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8979(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k) = read_values_as!(read_line(reader), usize, usize);

    let mut countries = Vec::with_capacity(n);
    for _ in 0..n {
        let (id, gold, silver, bronze) = read_values_as!(read_line(reader), usize, i32, i32, i32);
        countries.push((id, gold, silver, bronze));
    }

    // 금, 은, 동 순으로 정렬
    countries.sort_by(|a, b| b.1.cmp(&a.1).then(b.2.cmp(&a.2)).then(b.3.cmp(&a.3)));

    let &(_, target_gold, target_silver, target_bronze) =
        countries.iter().find(|&&(id, _, _, _)| id == k).unwrap();

    let rank = countries
        .iter()
        .filter(|&&(_, g, s, b)| {
            g > target_gold
                || (g == target_gold && s > target_silver)
                || (g == target_gold && s == target_silver && b > target_bronze)
        })
        .count()
        + 1;

    writeln!(writer, "{}", rank).unwrap();
}

// https://www.acmicpc.net/problem/8979
// 올림픽
#[test]
fn test_solve8979() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 3
1 1 2 0
2 0 1 0
3 0 1 0
4 0 0 1"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4 2
1 3 0 0
3 0 0 2
4 0 2 0
2 0 2 0"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1 1
1 0 0 0
"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "4 3
1 0 0 0
2 0 0 0
3 0 0 0
4 0 0 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "4 4
1 1 0 0
2 2 0 0
3 0 0 0
4 1 0 0"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5 5
1 2 0 0
2 2 1 0
3 1 2 1
4 2 0 1
5 2 1 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "4 4
1 10 0 0
2 0 5 0
3 0 0 5
4 0 0 1"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5 5
1 2 0 0
2 1 1 1
3 1 1 1
4 0 5 0
5 0 1 5"
                .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8979(&mut reader, &mut writer);

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
