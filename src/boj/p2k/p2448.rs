use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2448(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut board = vec![vec![' '; 2 * n - 1]; n];
    draw_star(&mut board, 0, n - 1, n);

    let result = board
        .into_iter()
        .map(|row| row.into_iter().collect::<String>().trim_end().to_string())
        .collect::<Vec<String>>()
        .join("\n");

    write!(writer, "{}", result).unwrap();
}

fn draw_star(board: &mut Vec<Vec<char>>, y: usize, x: usize, n: usize) {
    if n == 3 {
        board[y][x] = '*';
        board[y + 1][x - 1] = '*';
        board[y + 1][x + 1] = '*';
        board[y + 2][x - 2..=x + 2].fill('*');
        return;
    }

    let m = n / 2;
    draw_star(board, y, x, m);
    draw_star(board, y + m, x - m, m);
    draw_star(board, y + m, x + m, m);
}

// https://www.acmicpc.net/problem/2448
// 별 찍기 - 11
#[test]
fn test_solve2448() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3".to_string(),
            want: "  *
 * *
*****"
                .to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "     *
    * *
   *****
  *     *
 * *   * *
***** *****"
                .to_string(),
        },
        TestData {
            s: "12".to_string(),
            want: "           *
          * *
         *****
        *     *
       * *   * *
      ***** *****
     *           *
    * *         * *
   *****       *****
  *     *     *     *
 * *   * *   * *   * *
***** ***** ***** *****"
                .to_string(),
        },
        TestData {
            s: "24".to_string(),
            want: "                       *
                      * *
                     *****
                    *     *
                   * *   * *
                  ***** *****
                 *           *
                * *         * *
               *****       *****
              *     *     *     *
             * *   * *   * *   * *
            ***** ***** ***** *****
           *                       *
          * *                     * *
         *****                   *****
        *     *                 *     *
       * *   * *               * *   * *
      ***** *****             ***** *****
     *           *           *           *
    * *         * *         * *         * *
   *****       *****       *****       *****
  *     *     *     *     *     *     *     *
 * *   * *   * *   * *   * *   * *   * *   * *
***** ***** ***** ***** ***** ***** ***** *****"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2448(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
