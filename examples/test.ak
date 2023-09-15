use std::fs;

let content = fs.read("readme.md");

const person = {
    name: "aidin",
    age: 20,
    other: {
        address: "joben",
        email: "aidin@gmail.com" 
    }
};

fn rev_str(str) {
    ret str.chars().rev().join("");
}

println(
    content
);
