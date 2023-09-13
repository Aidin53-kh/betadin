fn new_user(name, email) {
    ret {
        name: name,
        email: email,
    };
}

const person = {
    name: "aidin53",
    data: {
        email: "aidn53@gmail.com",
        password: 123445667,
    }
};

let name = person.name;
let data = person.get("data");
let email = person.data.get("email");
let pass_len = person.data.password.to_string().len();

println(name);
println(data);
println(email);
println(pass_len);

println("-------------------------");

let user = new_user("user1", "user1@gmail.com");

println(user.name);
println(user.email);
