mod obfuscator;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 3 {
        obfuscator::run(args);
    } else {
        println!("usage: obfus [input.cpp] [output.cpp]")
    }
}
