let x = fn() {
    return module {
        const name = "aidin";
    };
};

x = x();

let y = if x::name.len() > 5 {
    return "if block";
} else {
    return "else block";
};

println(y);
