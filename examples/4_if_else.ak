# 2.1: if statements
let x = 3;
if x == 5 {
    println("x is 5");
} else if x > 5 {
    println("x is greater then 5");
} else {
    println("x is less then 5");
}



# 2.2 if expressions
let num = 10;
let y = if num == 10 {
    return num * 2;
} else {
    return num;
};
println(y); # 20
