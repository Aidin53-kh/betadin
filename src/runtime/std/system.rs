use crate::runtime::value::Value;
use std::env;

pub fn _platform(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    Ok(Value::String(env::consts::OS.to_string()))
}

pub fn _free_mem(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let mem_info = sys_info::mem_info().unwrap();
    Ok(Value::Int(mem_info.free as i32))
}

pub fn _total_mem(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let mem_info = sys_info::mem_info().unwrap();
    Ok(Value::Int(mem_info.total as i32))
}

pub fn _total_disk(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let disk_info = sys_info::disk_info().unwrap();
    Ok(Value::Int(disk_info.total as i32))
}

pub fn _free_disk(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let disk_info = sys_info::disk_info().unwrap();
    Ok(Value::Int(disk_info.free as i32))
}

pub fn _cpus(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let cpu_num = sys_info::cpu_num().unwrap();
    Ok(Value::Int(cpu_num as i32))
}

pub fn _cpu_speed(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let cpu_speed = sys_info::cpu_speed().unwrap();
    Ok(Value::Int(cpu_speed as i32))
}

pub fn _version(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let version = sys_info::os_release().unwrap();
    Ok(Value::String(version))
}

pub fn _processes(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let processes = sys_info::proc_total().unwrap();
    Ok(Value::Int(processes as i32))
}

pub fn _arch(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    Ok(Value::String(env::consts::ARCH.to_string()))
}

pub fn _family(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    Ok(Value::String(env::consts::FAMILY.to_string()))
}
