fn range(start, end) {
    let res = [];
    if start > end {
        panic("invalid arguments: starts most be lower then end");
    }
    
    while start < end {
        res = res.push(start);
        start = start + 1;
    }

    ret res;
}

for i in range(1, 10) {
    println(i);
}