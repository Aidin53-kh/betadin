import core::fs; # custom module
# import std::fs; # builtin module

let file = fs::read_file(2);

if file.is_ok {
    println(file.content);
} else {
    println("error: " + file.error);
}

