#[allow(unused_imports)]
use proconio::input;

struct InputData {
    a: f32,
    b: f32,
    n: f32,
}
fn main() {
    input! { a: f32, b: f32, n: f32 };
    let result = max_floor(InputData{a, b, n});
    println!("{}", result);
}


fn max_floor(data: InputData) -> i32 {
    let InputData {a, b, n} = data;
    if b - 1.0 < n {
        return (((a * (b - 1.0)).floor() / b) - (a * ((b - 1.0) / b).floor())) as i32;
    }
    (((a * n) / b).floor() - (a * (n / b).floor())) as i32
}

#[test]
fn qa() {
    let answer = 2;
    let data = InputData{a: 5.0, b: 7.0, n: 4.0};
    let result = max_floor(data);
    if result != answer {
        panic!("not be result : {} expect {}", result, answer)
    }
}

#[test]
fn qb() {
    let answer = 9;
    let data = InputData{a: 11.0, b: 10.0, n: 9.0};
    let result = max_floor(data);
    if result != answer {
        panic!("not be result : {} expect {}", result, answer)
    }
}
