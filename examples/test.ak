type output = bool;
type Fn = fn(int) -> output[];

let x: Fn = fn(g: int) {
    return false; # Error: expected `fn(int) -> bool[]` found `fn(int) -> bool`
};
