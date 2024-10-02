use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25703(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut ans = String::with_capacity(n * 20);

    ans.push_str("int a;\n");
    for i in 1..=n {
        let ptr = "*".repeat(i);
        let curr_cnt = if i == 1 { "ptr" } else { &format!("ptr{}", i) };
        let prev_cnt = match i {
            1 => "&a",
            2 => "&ptr",
            _ => &format!("&ptr{}", i - 1),
        };
        ans.push_str(&format!("int {}{} = {};\n", ptr, curr_cnt, prev_cnt));
    }

    ans.pop(); // remove last newline

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/25703
// 포인터 공부
#[test]
fn test_solve25703() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "int a;
int *ptr = &a;"
                .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "int a;
int *ptr = &a;
int **ptr2 = &ptr;
int ***ptr3 = &ptr2;
int ****ptr4 = &ptr3;"
                .to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "int a;
int *ptr = &a;
int **ptr2 = &ptr;
int ***ptr3 = &ptr2;
int ****ptr4 = &ptr3;
int *****ptr5 = &ptr4;
int ******ptr6 = &ptr5;
int *******ptr7 = &ptr6;
int ********ptr8 = &ptr7;
int *********ptr9 = &ptr8;"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25703(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
