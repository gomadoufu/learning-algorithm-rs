use proconio::input;
fn main() {
    //入力を受け取る
    input! {
        n: usize,
        v: isize,
        a: [isize; n],
    }

    //線形探索
    let mut exist = false;
    for i in a {
        if i == v {
            exist = true;
        }
    }

    //結果出力
    if exist {
        println!("Yes");
    } else {
        println!("No");
    }
}
