fn aidin() {
    return module {
        const name = "aidin53";
    };
}

let x = aidin();

println(x::name);

# import std::system;

# module aidin {
#     const name = "aidin53";

#     module data {
#         fn getage() {
#             return 20;
#         }
#     }
# }

# import aidin::data;

# const myname = aidin::name;
# const name = data::getage();

# fn main() {
#     import std::system;

#     println(system::platform().chars().rev().join(""));
# }

# main();

# println(myname);
# println(name);
# println(system::platform());
