use crate::read_values_as;
use crate::utils::io::{read_line, read_map};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14503(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (rows, cols) = read_values_as!(read_line(reader), usize, usize);
    let (start_row, start_col, start_dir) = read_values_as!(read_line(reader), usize, usize, usize);

    let map = read_map(reader, rows, cols);

    let mut cleaner = Cleaner::new(start_col, start_row, start_dir, map);
    cleaner.clean();

    write!(writer, "{}", cleaner.count).expect("Failed to write");
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_index(index: usize) -> Self {
        use Direction::*;
        match index {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => unreachable!(),
        }
    }

    fn left(self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn back(self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    fn delta(self) -> (isize, isize) {
        use Direction::*;
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}

struct Cleaner {
    x: usize,
    y: usize,
    direction: Direction,
    map: Vec<Vec<usize>>,
    count: usize,
}

impl Cleaner {
    fn new(x: usize, y: usize, direction_index: usize, map: Vec<Vec<usize>>) -> Self {
        let direction = Direction::from_index(direction_index);

        Self {
            x,
            y,
            direction,
            map,
            count: 0,
        }
    }

    fn clean(&mut self) {
        loop {
            self.clean_current();

            if !self.has_cleanable_around() {
                if !self.try_to_rewind() {
                    break;
                }
                continue;
            }

            self.turn_left();
            self.go_forward();
        }
    }

    fn clean_current(&mut self) {
        if self.map[self.y][self.x] == 0 {
            self.map[self.y][self.x] = 2;
            self.count += 1;
        }
    }

    fn has_cleanable_around(&self) -> bool {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .iter()
        .any(|&d| {
            let (dx, dy) = d.delta();
            self.is_cleanable(self.x, self.y, dx, dy)
        })
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.left();
    }

    fn go_forward(&mut self) {
        let (dx, dy) = self.direction.delta();
        if self.is_cleanable(self.x, self.y, dx, dy) {
            self.x = self.x.checked_add_signed(dx).unwrap();
            self.y = self.y.checked_add_signed(dy).unwrap();
        }
    }

    fn try_to_rewind(&mut self) -> bool {
        let (dx, dy) = self.direction.back().delta();
        if self.is_rewindable(self.x, self.y, dx, dy) {
            self.x = self.x.checked_add_signed(dx).unwrap();
            self.y = self.y.checked_add_signed(dy).unwrap();
            true
        } else {
            false
        }
    }

    fn try_next_pos(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
        let nx = x.checked_add_signed(dx)?;
        let ny = y.checked_add_signed(dy)?;

        if nx < self.map[0].len() && ny < self.map.len() {
            Some((nx, ny))
        } else {
            None
        }
    }

    fn is_cleanable(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        self.try_next_pos(x, y, dx, dy)
            .map(|(nx, ny)| self.map[ny][nx] == 0)
            .unwrap_or(false)
    }

    fn is_rewindable(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        self.try_next_pos(x, y, dx, dy)
            .map(|(nx, ny)| self.map[ny][nx] != 1)
            .unwrap_or(false)
    }
}

// https://www.acmicpc.net/problem/14503
// 로봇 청소기
#[test]
fn test_solve14503() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 3
1 1 0
1 1 1
1 0 1
1 1 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "11 10
7 4 0
1 1 1 1 1 1 1 1 1 1
1 0 0 0 0 0 0 0 0 1
1 0 0 0 1 1 1 1 0 1
1 0 0 1 1 0 0 0 0 1
1 0 1 1 0 0 0 0 0 1
1 0 0 0 0 0 0 0 0 1
1 0 0 0 0 0 0 1 0 1
1 0 0 0 0 0 1 1 0 1
1 0 0 0 0 0 1 1 0 1
1 0 0 0 0 0 0 0 0 1
1 1 1 1 1 1 1 1 1 1"
                .to_string(),
            want: "57".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14503(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
