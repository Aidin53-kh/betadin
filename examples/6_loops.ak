# 6.1: while loop
let x = 0;
while x < 6 {
    if x == 2 {
        x = x + 1;
        continue;
    } else if x == 4 {
        break;
    }
    
    println(x); # output: 0, 1, 3
    x = x + 1;
}

# 6.2: for loop
let colors = ["red", "green", "blue"];
for i in colors {
    println(i); # output: "red", "green", "blue"
}

for i in 1..5 {
    println(i); # output: 1, 2, 3, 4, 5
}
