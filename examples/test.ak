use std::env;
use std::fs;

env.set_var("AK", "aidin53");

println(env.var("AK"));

env.remove_var("AK");

println(env.var("AK"));
