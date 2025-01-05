use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10431(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases: usize = read_value(read_line(reader));
    for i in 1..=num_cases {
        let mut nums: Vec<usize> = read_n_values(reader, 21);
        let case_num = nums.remove(0);
        assert_eq!(case_num, i);

        let mut steps = 0;
        let mut lineup = Vec::with_capacity(20);

        // 각 학생을 오름차순 유지 상태에 삽입하면서
        // 삽입하는 위치 뒤의 학생들이 뒤로 물러난 횟수를 누적
        for num in nums {
            let pos = lineup.partition_point(|&x| x < num);
            steps += lineup.len() - pos;
            lineup.insert(pos, num);
        }

        writeln!(writer, "{} {}", case_num, steps).unwrap();
    }
}

// https://www.acmicpc.net/problem/10431
// 줄세우기
#[test]
fn test_solve10431() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
1 900 901 902 903 904 905 906 907 908 909 910 911 912 913 914 915 916 917 918 919
2 919 918 917 916 915 914 913 912 911 910 909 908 907 906 905 904 903 902 901 900
3 901 902 903 904 905 906 907 908 909 910 911 912 913 914 915 916 917 918 919 900
4 918 917 916 915 914 913 912 911 910 909 908 907 906 905 904 903 902 901 900 919"
                .to_string(),
            want: "1 0
2 190
3 19
4 171"
                .to_string(),
        },
        TestData {
            s: "1
1 900 902 901 905 903 904 906 907 908 909 910 911 912 913 914 915 916 917 918 919
"
            .to_string(),
            want: "1 3
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10431(&mut reader, &mut writer);

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
