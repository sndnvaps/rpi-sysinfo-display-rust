use execute::Execute;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
extern crate alloc;

//rust f32 custom decimal point length pick up from
//https://zhuanlan.zhihu.com/p/466389032
trait F32Utils {
    fn round_fixed(self, n: u32) -> f32;
}

impl F32Utils for f32 {
    fn round_fixed(self, n: u32) -> f32 {
        if n <= 0 {
            return self.round();
        }
        let i = 10_usize.pow(n) as f32;
        let x = self * i;
        if self > 0_f32 {
            // 正数情况下 1.15_f32.round() 为1.2
            let m = x.round() as u32;
            m as f32 / i
        } else {
            //默认的负数round四舍五入取整(a) -1.15_f32.round() 为 -1.2 (b)
            let mr = x.trunc(); //整数部分
            let mf = x.fract(); //小数部分
            if mf.abs() >= 0.5 {
                // -3.14159 四舍五入保留3位 则-3141.59 / 1000 = -3.14159(逢五进一) 变为-3.140
                return (mr + 1_f32) / i;
            }
            //小数部分 < 0.5直接舍弃小数部分；小数点直接使用整数部分向前移动n位
            mr / i
        }
    }
}

pub fn get_cpu_temp() -> String {
    let mut command = execute::command_args!("cat", "/sys/class/thermal/thermal_zone0/temp");

    command.stdout(Stdio::piped());

    let output = command.execute_output().unwrap();
    let cpu_temp_str = String::from_utf8(output.stdout).unwrap();
    //println!("{}",cpu_temp_str.trim()); trim()用于去除换行符 '\n'
    let cpu_temp = cpu_temp_str.trim().parse::<i32>().unwrap() / 1000;

    return cpu_temp.to_string();
}

pub struct MemInfo {
    pub total: u32,
    pub free: u32,
}

struct CpuOccupy {
    user: u32,
    nice: u32,
    system: u32,
    idle: u32,
    iowait: u32,
    irq: u32,
    softirq: u32,
}

pub fn get_ram() -> MemInfo {
    let mut command = execute::command_args!("head", "-c", "83", "/proc/meminfo");

    command.stdout(Stdio::piped());

    let output = command.execute_output().unwrap();
    let meminfo = String::from_utf8(output.stdout).unwrap();

    let (mem_total, mem_free) = scan_fmt!(
        &meminfo,                                             //input string
        "MemTotal:         {d} kB\nMemFree:          {d} kB", //format
        u32,
        u32
    )
    .unwrap(); //types
               //println!("MemTotal {} kB,MemFree {} kB",mem_total,mem_free);
    let mem_info = MemInfo {
        total: mem_total,
        free: mem_free,
    };

    return mem_info;
}

fn get_cpu_occupy() -> CpuOccupy {
    let mut command = execute::command_args!("head", "-c", "100", "/proc/stat");

    command.stdout(Stdio::piped());

    let output = command.execute_output().unwrap();
    let stat_info = String::from_utf8(output.stdout).unwrap();
    let (user, nice, system, idle, iowait, irq, softirq) = scan_fmt!(
        &stat_info,                          //input string
        "cpu  {d} {d} {d} {d} {d} {d} {d} ", //format
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32
    )
    .unwrap(); //types
    //println!("cpu {} {} {} {} {} {} {} ",user,nice,system,idle,iowait,irq,softirq);

    let cpu_info = CpuOccupy {
        user,
        nice,
        system,
        idle,
        iowait,
        irq,
        softirq,
    };

    return cpu_info;
}

fn cal_cpuoccupy(c1: CpuOccupy, c2: CpuOccupy) -> f32 {
    let mut cpu_use: f32 = 0.0;
    let od = (c1.user + c1.nice + c1.system + c1.idle + c1.softirq + c1.iowait + c1.irq) as f32; //第一次(用户+优先级+系统+空闲)的时间再赋给od
    let nd = (c2.user + c2.nice + c2.system + c2.idle + c2.softirq + c2.iowait + c2.irq) as f32; //第二次(用户+优先级+系统+空闲)的时间再赋给nd

    let id = c2.idle as f32; //用户第一次和第二次的时间之差再赋给id
    let sd = c1.idle as f32; //系统第一次和第二次的时间之差再赋给sd
    let sum = nd - od;
     //println!("od = {}, nd = {}, id = {} ,sd = {}",od,nd,id,sd);
    if sum != 0.0 {
        let idle = id - sd;
        cpu_use = 100_f32 - (idle / sum) * 100.0;
        cpu_use = cpu_use.round_fixed(2);
        println!("cpu_use = {}", cpu_use);
    }

    return cpu_use;
}

pub fn get_sys_cpu_usage() -> f32 {
    let mut cpu_usage: f32 = 0.0;
    let cpu_stat1 = get_cpu_occupy();
    thread::sleep(Duration::from_secs(1));
    let cpu_stat2 = get_cpu_occupy();
    cpu_usage = cal_cpuoccupy(cpu_stat1, cpu_stat2);
    return cpu_usage;
}
