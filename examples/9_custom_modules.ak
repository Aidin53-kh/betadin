# 9.1: module statements
module custom {
    const name = "custom";

    module inner {
        const name = "inner";

        fn getname() {
            return custom::inner::name;
        }

        fn get_parent_name() {
            return custom::name;
        }
    }
}

println(custom::inner::get_parent_name()); # output: `custom`
println(custom::inner::getname()); # output: `inner`

# or 

import custom::inner::{get_parent_name, getname};

println(get_parent_name()); # output: `custom`
println(getname()); # output: `inner`


# 9.2: module expressions
let x = module {
    let y = 10;
};
println(x::y * 10); # 100
