fn get_name() {
    ret "aidin53";
}

const person = {
    name: get_name
};

let name = person.name();

println(name());
