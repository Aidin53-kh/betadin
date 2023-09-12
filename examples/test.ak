fn double(x) {
    ret x * 2;
}

fn set(arr) {
    use std::collections::set;

    ret set(arr);
}

println(set([1, 1, 2, 2, 3, 3, 4, 2, 1, 5, 7]));
println(double(7));

fn max(x, y) {
    if x > y {
        ret x;
    } else {
        ret y;
    }
}

println("the max of 3 and 5 = " + max(3, 5));
