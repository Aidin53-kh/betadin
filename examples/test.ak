use std::system;

println("free mem: " + system.free_mem());
println("total mem: " + system.total_mem());
println("total disk: " + system.total_disk() / 1024);
println("free disk: " + system.free_disk() / 1024);
println("platform: " + system.platform());
println("cpus: " + system.cpus());
println("cpu speed: " + system.cpu_speed());
println("processes: " + system.processes());



