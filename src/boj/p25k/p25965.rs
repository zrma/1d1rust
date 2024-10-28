use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25965(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));

    for _ in 0..num_cases {
        let n: usize = read_value(read_line(reader));

        let donations: Vec<_> = (0..n)
            .map(|_| read_values_as!(read_line(reader), i64, i64, i64))
            .collect();

        let (k_factor, d_factor, a_factor) = read_values_as!(read_line(reader), i64, i64, i64);

        let total_donation: i64 = donations.iter().fold(0, |acc, (k, d, a)| {
            let donation = k * k_factor - d * d_factor + a * a_factor;
            if donation > 0 {
                acc + donation
            } else {
                acc
            }
        });

        writeln!(writer, "{}", total_donation).expect("writeln! should work");
    }
}

// https://www.acmicpc.net/problem/25965
// 미션 도네이션
#[test]
fn test_solve25965() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
2
1000 2000 3000
1000 4000 1000
4 1 2
1
4000 3000 2000
2 4 2"
                .to_string(),
            want: "10000
0
"
            .to_string(),
        },
        TestData {
            s: "1
3
9146 6114 4301
7587 5074 9944
3175 9794 3119
1 5 5"
                .to_string(),
            want: "32018
"
            .to_string(),
        },
        TestData {
            s: "3
4
13239 13091 11619
10431 6300 10235
14340 8606 9048
12777 10975 14396
8 2 0
3
8937 9151 1868
13650 2597 1531
13746 6384 11839
4 0 9
2
3934 5773 8056
11015 5695 9285
13 6 14"
                .to_string(),
            want: "328352
282474
368303
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25965(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
