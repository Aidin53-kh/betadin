import std::system;

module aidin {
    const name = "aidin53";

    module data {
        fn getage() {
            return 20;
        }
    }
}

import aidin::data;

const myname = aidin::name;
const name = data::getage();

println(myname);
println(name);
println(system::platform());
