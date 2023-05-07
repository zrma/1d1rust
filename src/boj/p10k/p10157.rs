#[allow(dead_code)]
fn solve10157(col: i32, row: i32, k: i32) -> (i32, i32, bool) {
    if row * col < k {
        return (0, 0, false);
    }

    let mut x = 1;
    let mut y = 1;
    let mut k0 = k - 1;
    let mut col0 = col - 1;
    let mut row0 = row;
    let mut dir = Direction::Up;

    loop {
        match dir {
            Direction::Up => {
                if k0 < row0 {
                    return (x, y + k0, true);
                }
                x += 1;
                y += row0 - 1;
                k0 -= row0;
                row0 -= 1;
                dir = turn(dir);
            }
            Direction::Right => {
                if k0 < col0 {
                    return (x + k0, y, true);
                }
                x += col0 - 1;
                y -= 1;
                k0 -= col0;
                col0 -= 1;
                dir = turn(dir);
            }
            Direction::Down => {
                if k0 < row0 {
                    return (x, y - k0, true);
                }
                x -= 1;
                y -= row0 - 1;
                k0 -= row0;
                row0 -= 1;
                dir = turn(dir);
            }
            Direction::Left => {
                if k0 < col0 {
                    return (x - k0, y, true);
                }
                x -= col0 - 1;
                y += 1;
                k0 -= col0;
                col0 -= 1;
                dir = turn(dir);
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

fn turn(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

// https://www.acmicpc.net/problem/10157
#[test]
fn test_solve10157() {
    struct TestData {
        col: i32,
        row: i32,
        k: i32,
        want_ok: bool,
        want_x: i32,
        want_y: i32,
    }
    for data in vec![
        TestData {
            col: 7,
            row: 6,
            k: 1,
            want_ok: true,
            want_x: 1,
            want_y: 1,
        },
        TestData {
            col: 7,
            row: 6,
            k: 6,
            want_ok: true,
            want_x: 1,
            want_y: 6,
        },
        TestData {
            col: 7,
            row: 6,
            k: 7,
            want_ok: true,
            want_x: 2,
            want_y: 6,
        },
        TestData {
            col: 7,
            row: 6,
            k: 11,
            want_ok: true,
            want_x: 6,
            want_y: 6,
        },
        TestData {
            col: 7,
            row: 6,
            k: 12,
            want_ok: true,
            want_x: 7,
            want_y: 6,
        },
        TestData {
            col: 7,
            row: 6,
            k: 13,
            want_ok: true,
            want_x: 7,
            want_y: 5,
        },
        TestData {
            col: 7,
            row: 6,
            k: 17,
            want_ok: true,
            want_x: 7,
            want_y: 1,
        },
        TestData {
            col: 7,
            row: 6,
            k: 87,
            want_ok: false,
            want_x: 0,
            want_y: 0,
        },
        TestData {
            col: 100,
            row: 100,
            k: 3000,
            want_ok: true,
            want_x: 9,
            want_y: 64,
        },
        TestData {
            col: 10000,
            row: 10000,
            k: 99999999,
            want_ok: true,
            want_x: 5001,
            want_y: 5001,
        },
    ] {
        let (got_x, got_y, got_ok) = solve10157(data.col, data.row, data.k);
        assert_eq!(got_ok, data.want_ok);
        if got_ok {
            assert_eq!(got_x, data.want_x);
            assert_eq!(got_y, data.want_y);
        }
    }
}
