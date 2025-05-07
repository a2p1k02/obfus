use rand::distr::Distribution;
use std::collections::HashMap;
use std::io::{Read, Write};
use rand::distr::Uniform;
use rand::Rng;
use regex::Regex;

fn generate_random_name() -> String {
    let ascii_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
    let all_chars = format!("{}{}", ascii_chars, "0123456789").chars().collect::<Vec<char>>();

    let mut rng = rand::rng();
    let length = rng.random_range(7..=16);
    let mut random_name: Vec<char> = Vec::new();

    random_name.push(ascii_chars.chars().collect::<Vec<char>>()
        [rng.random_range(0..ascii_chars.len())]);

    let between = Uniform::try_from(0..all_chars.len()).unwrap();
    
    for _ in 1..length {
        random_name.push(all_chars[between.sample(&mut rng)]);
    }

    random_name.iter().collect()
}

fn obfuscate_string(code: &str) -> String {
    let re = Regex::new(r#"(?s)([^\\]|^)(")(.*?)(")"#).unwrap();
    re.replace_all(code, |caps: &regex::Captures| {
        if caps.len() < 5 {
            return caps[0].to_string();
        }

        if caps[0].trim_start().starts_with('#') {
            return caps[0].to_string();
        }

        let s = &caps[3];
        if s.is_empty() {
            return caps[0].to_string();
        }

        if s.contains(r"\x") {
            return caps[0].to_string();
        }

        let mut hex_str = String::new();
        for c in s.chars() {
            if c == '\\' {
                hex_str.push(c);
                continue;
            }
            hex_str.push_str(&format!("\\x{:02x}", c as u32));
        }

        format!("{}{}\"{}\"", &caps[1], &caps[2], hex_str)
    }).to_string()
}

fn obfuscate_numbers(code: &str) -> String {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.replace_all(code, |caps: &regex::Captures| {
        if caps[0].contains('.') {
            return caps[0].to_string();
        }

        let num: i32 = caps[0].to_string().parse().unwrap();
        match num {
            0 => "0x0".to_string(),
            1 => "0b1".to_string(),
            _ => format!("0x{}", num)
        }
    }).to_string()
}

fn obfuscate_cpp_code(code: &str) -> String {
    let mut replacements: HashMap<String, String> = HashMap::new();

    let reserved: HashMap<String, bool> = [
        ("iostream", true), ("fstream", true), ("thread", true), ("windows.h", true),
        ("string", true), ("cout", true), ("cin", true), ("endl", true), ("main", true),
        ("std", true), ("to_string", true), ("ofstream", true), ("MAX_PATH", true),
        ("snprintf", true), ("join", true), ("for", true), ("close", true), ("sizeof", true),
    ].iter().map(|(k, v)| (k.to_string(), *v)).collect();

    let patterns = vec! [
        (Regex::new(r"\b(class|struct|void|int|bool|char|double|float|auto)\s+([a-zA-Z_]\w*)").unwrap(), 2), // types
        (Regex::new(r"\b([a-zA-Z_]\w*)\s*\(").unwrap(), 1),                                                  // functions
        (Regex::new(r"\b([a-zA-Z_]\w*)\s*=").unwrap(), 1),                                                   // vars 1
        (Regex::new(r"\b([a-zA-Z_]\w*)\s*;").unwrap(), 1),                                                   // vars 2
        (Regex::new(r"\b([a-zA-Z_]\w*)\s*:").unwrap(), 1),                                                   // methods
        (Regex::new(r"\b([a-zA-Z_]\w*)\s*\{").unwrap(), 1),                                                  // code
        (Regex::new(r"\b([a-zA-Z_]\w*)\s*\)").unwrap(), 1),                                                  // args
    ];

    for (regex, group) in patterns {
        for cap in regex.captures_iter(code) {
            if let Some(identifier) = cap.get(group) {
                let identifier = identifier.as_str().to_string();
                if !reserved.contains_key(&identifier) && !replacements.contains_key(&identifier) {
                    replacements.insert(identifier, generate_random_name());
                }
            }
        }
    }

    let mut modified_code = code.to_string();
    for (old, new) in replacements {
        let re = Regex::new(&format!(r"\b{}\b", regex::escape(&old))).unwrap();
        modified_code = re.replace_all(&modified_code, new.as_str()).to_string();
    }

    modified_code = obfuscate_string(&modified_code);
    modified_code = obfuscate_numbers(&modified_code);

    let rand_num = rand::rng().random_range(1000..=9999);
    let rand_hex = format!("{:X}", rand::rng().random_range(0x1000..=0xFFFF));

    let garbage_code = format!(
        r#"#ifdef __GNUC__
#define __UNUSED __attribute__((unused))
#else
#define __UNUSED
#endif

template<typename T>
__UNUSED static inline void ___func_{}() {{
volatile T ___var_{} = (T)0x{};
___var_{} += (T)0x{};
}}
    "#, rand_num, rand_num, rand_hex, rand_num, rand_hex);

    format!("{}\n{}", garbage_code, modified_code)
}

pub fn run(args: Vec<String>) {
    let input_file = std::fs::File::open(args[1].to_string());
    let mut input_file_contents = String::new();
    input_file.unwrap().read_to_string(&mut input_file_contents).unwrap();

    let output_file_contents = obfuscate_cpp_code(input_file_contents.as_str());
    let output_file = std::fs::File::create(args[2].to_string());
    output_file.unwrap().write_all(output_file_contents.as_bytes()).unwrap();
}