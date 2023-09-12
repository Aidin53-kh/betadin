fn main(s, e) {
    let x = s;

    while x < e {  
        if x == 3 {
            x = x + 1;
            println("skip 3");
            continue;
        }
        if x == 9 {
            break;
        }
        println(x);
        x = x + 1;
    }
}

main(1, 12);

# for i in [1, 2, 3, 4, 5, 6, 7] {
#     if i == 3 {
#         continue;
#     }

#     println(i);
# }
