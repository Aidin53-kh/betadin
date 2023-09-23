# 8.1: import methods
import std::system;
println(system::platform());

# or
import std::system::{platform};
println(platform());

# or 
println(std::system::platform());


# 8.2: std::fs functions
import std::fs;

fs::read_file(path); # read file
fs::read_dir(path); # read directory
fs::remove_file(path); # remove file
fs::remove_dir(path); # remove directory
fs::rename_file(path, new_path); # rename file
fs::write_file(path, data); # write file


# 8.3: std::system functions
import std::system;

system::platform(); # system platform
system::free_mem(); # free memory
system::total_mem(); # total memory
system::free_disk(); # free disk
system::total_disk(); # total disk
system::cpu_speed(); # cpu speed
system::cpus(); # cpu quantity
system::arch(); # system architecture
system::version(); # system version
system::processes(); # processes
system::family(); # system family

# 8.4: std::env functions
import std::env;

env::args(); # get all program args
env::var(key); # get by key
env::vars(); # get all env variables
env::remove_var(key); # remove by key
env::set_var(key, value); # set env

# 8.5: std::collections functions
import std::collections;

# remove duplicated items
collections::set([1, 2, 2, 3, 1, 3]) # returns [1, 2, 3]

