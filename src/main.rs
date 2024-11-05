use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(version)]
struct Cli {
    /// File path to be processed
    #[clap(short, long, required = true)]
    path: String,

    /// Symbol used for substitution
    #[clap(short, long, default_value = "?")]
    symbol: char,

    /// Number of paths to display
    #[clap(short, long, default_value_t = 100)]
    number: usize,
}

fn main() {
    let cli = Cli::parse();
    let mut last_valid_fuzzed_path = String::new();
    
    let raw_result = get_item_path(&cli.path); // Cache the raw_result here

    fuzz_path(&cli.path, cli.symbol, cli.number, &mut last_valid_fuzzed_path, &raw_result);
}

fn fuzz_path(path: &str, symbol: char, number: usize, last_valid_fuzzed_path: &mut String, _raw_result: &str) {
    let mut counter = 0;
    let mut fuzzed_path = path.to_string().chars().collect::<Vec<char>>();

    // First phase: Substituting characters with the specified symbol
    for i in 0..path.len() {
        if counter >= number {
            break;
        }

        if path.chars().nth(i).unwrap().is_ascii_alphanumeric() || path.chars().nth(i).unwrap() == '?' {
            let original_char = fuzzed_path[i];
            fuzzed_path[i] = symbol;

            let fuzzed_path_str: String = fuzzed_path.iter().collect();

            if validate_path(&fuzzed_path_str, path) {
                println!("{}", fuzzed_path_str);
                *last_valid_fuzzed_path = fuzzed_path_str.clone();
                counter += 1;
            } else {
                fuzzed_path[i] = original_char;
            }
        }
    }

    // Second phase: Substituting other characters with '*'
    let mut i = 0;
    while counter < number && i < path.len() {
        if path.chars().nth(i).unwrap() != ':' && path.chars().nth(i).unwrap() != '\\' && path.chars().nth(i).unwrap() != '.' {
            let original_char = fuzzed_path[i];
            fuzzed_path[i] = '*';

            let fuzzed_path_str: String = fuzzed_path.iter().collect();

            if validate_path(&fuzzed_path_str, path) {
                println!("{}", fuzzed_path_str);
                *last_valid_fuzzed_path = fuzzed_path_str.clone();
                counter += 1;
            } else {
                fuzzed_path[i] = original_char;
            }
        }
        i += 1;
    }

// Continue with '*' substitutions one by one
let mut i = 0;
while counter < number && i < fuzzed_path.len() {
    if fuzzed_path[i] == '*' {
        let original_char = fuzzed_path.remove(i);
        let fuzzed_path_str: String = fuzzed_path.iter().collect();
    
        if validate_path(&fuzzed_path_str, path) {
            println!("{}", fuzzed_path_str);
            *last_valid_fuzzed_path = fuzzed_path_str.clone();
            counter += 1;
        } else {
            fuzzed_path.insert(i, original_char);
            i += 1; // Increment 'i' only after re-inserting the asterisk
        }

        // Check if 'i' is now out of bounds
        if i >= fuzzed_path.len() {
            break;
        }
    } else {
        i += 1; // Increment 'i' if current character is not an asterisk
    }
}

    // Fourth phase: Replace '?' with '*'
    let mut i = 0;
    while counter < number && i < fuzzed_path.len() {
        if fuzzed_path[i] == '?' {
            fuzzed_path[i] = '*';

            let fuzzed_path_str: String = fuzzed_path.iter().collect();

            if validate_path(&fuzzed_path_str, path) {
                println!("{}", fuzzed_path_str);
                *last_valid_fuzzed_path = fuzzed_path_str.clone();
                counter += 1;
            } else {
                // Revert back to '?' if the replacement is invalid
                fuzzed_path[i] = '?';
            }
        }
        i += 1;
    }

}



fn validate_path(fuzzed_path: &str, raw_result: &str) -> bool {
    let fuzzed_result = get_item_path(fuzzed_path);

    // Compare the results
    raw_result == fuzzed_result
}

fn get_item_path(path: &str) -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("(Get-Item -Path '{}').FullName", path))
        .output()
        .expect("Failed to execute PowerShell");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}