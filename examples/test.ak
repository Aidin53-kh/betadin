use std::fs::write;

const content = "my name is aidin53";

write("./examples/fs.ak", content);

# use std::fs::read_file;

# let file = read_file("./examples/test.ak");

# println(file.len());
