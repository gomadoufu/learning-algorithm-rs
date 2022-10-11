use proconio::input;
pub fn linear_search() {
    input! {
        N: usize,
        v: isize,
    }

    //入力を受け取る
    let mut a = Vec::<isize>::with_capacity(N);

    //線形探索
    let mut exist = false;
    for i in 0..N {
        input! {
            a: isize,
        }
        if a == v {
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
