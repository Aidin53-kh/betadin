# 3.1: function declarations
fn factorial(x) {
    if x < 0 {
        return -1;
    } else if (x == 0) {
        return 1;
    }
    
    return x * factorial(x - 1);
}
println(factorial(5)); # 120



# 3.2: function expressions
let add = fn(x, y) {
    return x + y;
};
println(add(4, 5)); # 9
