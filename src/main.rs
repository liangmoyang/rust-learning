fn main() {
    let n0;
    let n1;

    if (return_num() > 1) {
        n0 = 10;
        n1 = 20;
    } else {
        n0 = 1;
        n1 = 2;
    }

    println!("{},{}", n0, n1);
}

fn return_num() -> i32 {
    10
}


