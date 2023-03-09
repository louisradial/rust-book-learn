fn main() {
    let mut recursive_result;
    let mut direct_result;
    let mut iterative_result;

    for i in 0..30 {
        recursive_result = recursive_fibonacci(i);
        direct_result = binet_fibonacci(i);
        iterative_result = iterative_fibonacci(i);
        println!("n = {i}\nrecursive: {recursive_result}\ndirect: {direct_result}\niterative: {iterative_result}\n");
    }
}

fn binet_fibonacci(n: i32) -> i32 {
    let g = (1.0 + f64::sqrt(5.0)) / 2.0;
    ((g.powi(n) - (-g).powi(-n)) / f64::sqrt(5.0)) as i32
}

fn recursive_fibonacci(n: i32) -> i32 {
    let result: i32;
    if n == 0 {
        result = 0;
    } else if n == 1 {
        result = 1;
    } else {
        result = recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2);
    }
    result
}

fn iterative_fibonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut pprev;
    let mut prev = 1;
    let mut next = 1;
    for _ in 2..n {
        pprev = prev;
        prev = next;
        next = pprev + prev;
    }
    next
}
