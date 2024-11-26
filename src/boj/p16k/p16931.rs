use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16931(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let mut board = [[0; 102]; 102];
    board.iter_mut().skip(1).take(n).for_each(|row| {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        row.iter_mut().skip(1).take(m).for_each(|col| {
            *col = read_value(iter.next().unwrap().to_string());
        });
    });

    let res = surface_area(board, n, m);

    write!(writer, "{}", res).expect("Failed to write");
}

fn surface_area(board: [[i32; 102]; 102], n: usize, m: usize) -> i32 {
    let mut area = 2 * n as i32 * m as i32;
    for i in 1..=n {
        for j in 1..=m {
            let h = board[i][j];
            if h > 0 {
                area += (h - board[i - 1][j]).max(0);
                area += (h - board[i + 1][j]).max(0);
                area += (h - board[i][j - 1]).max(0);
                area += (h - board[i][j + 1]).max(0);
            }
        }
    }

    area
}

// https://www.acmicpc.net/problem/16931
// 겉넓이 구하기
#[test]
fn test_solve16931() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1
1"
            .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "3 3
1 3 4
2 2 3
1 2 4"
                .to_string(),
            want: "60".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve16931(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
