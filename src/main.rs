use std::{collections::HashMap, env, fs, process::exit, time::Duration};
use local_ip_address::local_ip;

static TABLE: &[(&str, &[&str])] = &[
    ("arch", &[
        "\x1b[36m\x1b[1m       /\\         \x1b[35m",
        "\x1b[36m\x1b[1m      /  \\        \x1b[34mos     \x1b[0m",
        "\x1b[36m\x1b[1m     /\\   \\       \x1b[34mip     \x1b[0m",
        "\x1b[34m\x1b[1m    /      \\      \x1b[34mkernel \x1b[0m",
        "\x1b[34m\x1b[1m   /   ,,   \\     \x1b[34muptime \x1b[0m",
        "\x1b[34m\x1b[1m  /   |  |  -\\    \x1b[34m",
        "\x1b[34m\x1b[1m /_-''    ''-_\\   \x1b[34mmemory \x1b[0m"
    ]),
    ("debian", &[
        "\x1b[31m\x1b[1m      _____       \x1b[32m",
        "\x1b[31m\x1b[1m     /  __ \\      \x1b[31mos     \x1b[0m",
        "\x1b[31m\x1b[1m    |  /    |     \x1b[31mip     \x1b[0m",
        "\x1b[31m\x1b[1m    |  \\___-      \x1b[31mkernel \x1b[0m",
        "\x1b[31m\x1b[1m    -_            \x1b[31muptime \x1b[0m",
        "\x1b[31m\x1b[1m      --_         \x1b[31m",
        "\x1b[31m\x1b[1m                  \x1b[31mmemory \x1b[0m"
    ]),
    ("ubuntu", &[
        "\x1b[33m\x1b[1m          _       \x1b[34m",
        "\x1b[33m\x1b[1m      ---(_)      \x1b[33mos     \x1b[0m",
        "\x1b[33m\x1b[1m  _/  ---  \\      \x1b[33mip     \x1b[0m",
        "\x1b[33m\x1b[1m (_) |   |        \x1b[33mkernel \x1b[0m",
        "\x1b[33m\x1b[1m   \\  --- _/      \x1b[33muptime \x1b[0m",
        "\x1b[33m\x1b[1m      ---(_)      \x1b[33m",
        "\x1b[33m\x1b[1m                  \x1b[33mmemory \x1b[0m",
    ]),
];

fn main() {
    let mut os = "";
    match fs::read_to_string("/etc/os-release") {
        Ok(os_release) => {
            for line in os_release.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    let pretty_name = value.trim_matches('"');
                    os = match pretty_name.to_lowercase().as_str() {
                        s if s.contains("debian") => "debian",
                        s if s.contains("ubuntu") => "ubuntu",
                        s if s.contains("arch") => "arch",
                        _ => "Unknown",
                    };
                }
            }
        },
        Err(e) => {
            eprintln!("Couldn't read /etc/os-release: {}", e);
            exit(1)
        },
    }
    if os.is_empty() {
        eprintln!("Couldn't find the OS.");
    }

    let lines = TABLE.iter().find(|&&(k, _)| k == os).map(|&(_, v)| v).unwrap();

    match env::var("USER") {
        Ok(user) => print!("{}{}", lines[0], user),
        Err(e) => eprint!("{}[0mCouldn't read $USER: {}", lines[0], e),
    }
    match fs::read_to_string("/etc/hostname") {
        Ok(hostname) => print!("@{}", hostname),
        Err(e) => eprintln!("@Couldn't read /etc/hostname: {}", e),
    }

    match fs::read_to_string("/etc/os-release") {
        Ok(os_release) => {
            for line in os_release.lines() {
                if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
                    let pretty_name = value.trim_matches('"');
                    println!("{}{}", lines[1], pretty_name);
                }
            }
        },
        Err(e) => eprintln!("{}Couldn't read /etc/os-release: {}", lines[1], e),
    }


    match local_ip() {
        Ok(ip) => println!("{}{}", lines[2], ip),
        Err(_e) => println!("{}Disconnected", lines[2]),
    }

    match fs::read_to_string("/proc/sys/kernel/ostype") {
        Ok(ostype) => print!("{}{} ", lines[3], ostype.trim_end()),
        Err(e) => eprint!("{}Couldn't read /proc/sys/kernel/ostype: {}", lines[3], e),
    }
    match fs::read_to_string("/proc/sys/kernel/osrelease") {
        Ok(osrelease) => print!("{}", osrelease),
        Err(e) => println!("Couldn't read /proc/sys/kernel/osrelease: {}", e),
    }

    match fs::read_to_string("/proc/uptime") {
        Ok(raw_uptime) => {
            if let Some(first_part) = raw_uptime.split_whitespace().next() {
                if let Ok(seconds) = first_part.split('.').next().unwrap_or("0").parse::<u64>() {
                    let duration = Duration::from_secs(seconds);
                    let days = duration.as_secs() / 86400;
                    let hours = (duration.as_secs() / 3600) % 24;
                    let minutes = (duration.as_secs() / 60) % 60;
                    let seconds = duration.as_secs() % 60;

                    let uptime_string = format!(
                        "{}{}{}{}s",
                        if days > 0 { format!("{}d ", days) } else { String::new() },
                        if hours > 0 { format!("{}h ", hours) } else { String::new() },
                        if minutes > 0 { format!("{}m ", minutes) } else { String::new() },
                        seconds
                    );

                    println!("{}{}", lines[4], uptime_string.trim());
                }
            }
        }
        Err(e) => eprintln!("{}Couldn't read /proc/uptime: {}", lines[4], e),
    }

    match fs::read_to_string("/sys/class/power_supply/BAT0/status") {
        Ok(battery_status) => {
            print!("{}power  \x1b[0m{}", lines[5], battery_status.trim_end());
            
            match fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                Ok(battery_charge) => println!(" {}%", battery_charge.trim_end()),
                Err(_e) => {
                    match fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                        Ok(battery_charge) => println!(" {}%", battery_charge.trim_end()),
                        Err(e) => println!(" Couldn't read /sys/class/power_supply/BAT0/capacity: {}", e),
                    }
                },
            }
        },
        Err(_e) => {
            let mut cores: f32 = 1.0;
            match fs::read_to_string("/sys/devices/system/cpu/present") {
                Ok(cpu_present) => cores = cpu_present[2..].trim().parse::<f32>().unwrap_or(0.0) + 1.0,
                Err(e) => print!("Couldn't read /sys/devices/system/cpu/present: {}", e),
            }
            match fs::read_to_string("/proc/loadavg") {
                Ok(loadavg) => print!("{}cpu    \x1b[0m{}%\n", lines[5], (loadavg.split_whitespace().next().unwrap().parse::<f32>().expect("Couldn't parse /proc/loadavg") / (cores + 1.0) * 100.0).floor()),
                Err(e) => eprintln!("{}cpu    \x1b[0mCouldn't read /proc/loadavg: {}", lines[5], e),
            }
        },
    }
    
    match fs::read_to_string("/proc/meminfo") {
        Ok(meminfo) => {
            let mut mem_map = HashMap::new();

            for line in meminfo.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().to_string();
                    let value = value.trim().split_whitespace().next().unwrap_or("0");
                    if let Ok(parsed_value) = value.parse::<u64>() {
                        mem_map.insert(key, parsed_value);
                    }
                }
            }

            let sizes = ["KB", "MB", "GB", "TB"];

            let used_memory = (*mem_map.get("MemTotal").unwrap_or(&0)
            - *mem_map.get("MemFree").unwrap_or(&0)
            - *mem_map.get("Buffers").unwrap_or(&0)
            - *mem_map.get("Cached").unwrap_or(&0)
            - *mem_map.get("SReclaimable").unwrap_or(&0)) as f64;

            let mut size = used_memory;
            let mut unit = "KB";

            for &next_unit in &sizes {
                if size < 1024.0 {
                    unit = next_unit;
                    break;
                }
                size /= 1024.0;
            }

            print!("{}{:.1} {}", lines[6], size, unit);
            
            let mut size = *mem_map.get("MemTotal").unwrap_or(&0) as f64;
            let mut unit = "KB";

            for &next_unit in &sizes {
                if size < 1024.0 {
                    unit = next_unit;
                    break;
                }
                size /= 1024.0;
            }

            println!(" / {:.1} {}\n", size, unit);
        }
        Err(e) => print!("{}Couldn't read /proc/meminfo: {}c", lines[6], e),
    }
}


/*
					${c3}         _
					    ---(_)
					_/  ---  \\
					(_) |   |
					 \\  --- _/
					    ---(_)
				EOF

                at the start we find the os symbol and put it from a static array into a hashmap or something. then we can get it easily with just the index.
 */