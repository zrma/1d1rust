use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2002(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (in_cars, out_cars) = read_input(reader);

    let res = count_mismatched_order(&in_cars, &out_cars);

    write!(writer, "{}", res).expect("Failed to write");
}

fn read_input(reader: &mut impl BufRead) -> (Vec<String>, Vec<String>) {
    let n: usize = read_line(reader).parse().unwrap();
    let in_cars = read_cars(reader, n);
    let out_cars = read_cars(reader, n);

    (in_cars, out_cars)
}

fn read_cars(reader: &mut impl BufRead, n: usize) -> Vec<String> {
    (0..n).map(|_| read_line(reader)).collect()
}

fn count_mismatched_order(in_cars: &[String], out_cars: &[String]) -> usize {
    let mut checked = vec![false; in_cars.len()];
    let mut res = 0;

    for in_car in in_cars {
        let mut found = false;

        for (idx, out_car) in out_cars.iter().enumerate() {
            if in_car == out_car {
                checked[idx] = true;
                found = true;
                break;
            } else if !checked[idx] {
                checked[idx] = true;
                res += 1;
            }
        }

        if !found {
            break;
        }
    }

    res
}

// https://www.acmicpc.net/problem/2002
// 추월
#[test]
fn test_solve2002() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1
2
3
4
5
1
3
2
5
4"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4
ZG431SN
ZG5080K
ST123D
ZG206A
ZG206A
ZG431SN
ZG5080K
ST123D"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "5
ZG508OK
PU305A
RI604B
ZG206A
ZG232ZF
PU305A
ZG232ZF
ZG206A
ZG508OK
RI604B"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5
ZG206A
PU234Q
OS945CK
ZG431SN
ZG5962J
ZG5962J
OS945CK
ZG206A
PU234Q
ZG431SN"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4
a
b
c
d
d
a
c
b"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "10
7250SI4T
51475H
48642VM
F2T56T5K
7JK86BA
03L062
0073MM
Y0AD3H
K67T3B4
USF316
7250SI4T
0073MM
48642VM
F2T56T5K
7JK86BA
03L062
51475H
Y0AD3H
K67T3B4
USF316"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "20
66G776P8
361517
X20G2M
AO6C4D
V8O717LM
47B6322B
1280DH03
4AI1AC7
A3G550
38586T25
E75828
3M200VPN
7M02R0G0
877F43
24446M23
465HQ54
5013DC12
525745
G1288Y1
7005B8
24446M23
66G776P8
X20G2M
AO6C4D
47B6322B
V8O717LM
E75828
4AI1AC7
A3G550
465HQ54
G1288Y1
3M200VPN
7M02R0G0
877F43
361517
38586T25
5013DC12
525745
1280DH03
7005B8"
                .to_string(),
            want: "16".to_string(),
        },
        TestData {
            s: "50
73162A6U
587SI61
56V1608Y
321Q2V
X46827J
D8X336
271V57E2
7F2L3Y
QN3CR65T
122A0E2
670087
05Y62J
3708462
UL55R6
A5Q25027
T6M225
72JCL77
55G15003
FY3835
30VA046L
743A53
780822
KY66571
66J58U86
22613444
508KU6
180A151Q
4EC076K5
6S55151
00T08C4Y
25H0587
48544L1
44L323F4
6BGF45U
P7CVX462
643417GQ
4416W304
K242W204
D348L5B
638074
847G38
5L46W7
44B5D353
17Q231
63G33P2
52360E
PD3P2T8
27845R
K8O5WV61
4W732N
73162A6U
643417GQ
63G33P2
K242W204
X46827J
D8X336
847G38
7F2L3Y
743A53
122A0E2
PD3P2T8
05Y62J
3708462
4416W304
P7CVX462
27845R
72JCL77
K8O5WV61
FY3835
30VA046L
5L46W7
780822
587SI61
UL55R6
22613444
508KU6
48544L1
KY66571
6S55151
66J58U86
25H0587
180A151Q
44L323F4
6BGF45U
A5Q25027
321Q2V
00T08C4Y
D348L5B
4EC076K5
638074
55G15003
56V1608Y
44B5D353
17Q231
QN3CR65T
52360E
670087
T6M225
271V57E2
4W732N"
                .to_string(),
            want: "45".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2002(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
