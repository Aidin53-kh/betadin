# 7.1: example
let name = "betadin";
let rev_name = name.chars().rev().join(""); # reverse string
println(rev_name); # output: `nidateb`


# 7.2: type methods do not change the value of a variable
let nums = [1, 2, 3];
nums.push(4); # it does not work, it just return the edited value
# use this instead
nums = nums.push(4);
println(nums); # output: `[1, 2, 3, 4]`


# 7.2: integer type methods
let int = 1;
int.pow(int);
int.to_string();


# 7.3: float type methods
let float = 1.3;
float.pow(int);
float.to_string();


# 7.4: string type methods
let string = "something";
string.len();
string.to_string();
string.at(index);
string.chars();
string.split(str);
string.to_upper();
string.to_lower();
string.trim();
string.lines();
string.to_numeric();
string.is_ascii();
string.contains(str);
string.repeat(int);
string.replace(str_old, str_new);
string.push(str);


# 7.5: list type methods
let list = 1..10;
list.push(value);
list.pop();
list.at(index);
list.len();
list.rev();
list.join(str);
list.clear();
list.contains(value);
list.to_string();


# 7.6 object type methos
let object = {};
object.get(key);
object.set(key, value);
object.keys();
object.values();
object.remove(key);
object.contains(key)
object.clear();


# 7.7 others
let tuple = (1, 2);
tuple.at(index);
null.to_string();
