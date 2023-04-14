#[macro_use]
extern crate data_encoding_macro;
use data_encoding::Encoding;
use dirs::home_dir;
use std::fs;

use hmac::Hmac;
use lazy_static::lazy_static;
use pbkdf2::pbkdf2;
use regex::Regex;
use sha2::Sha256;
use std::path::PathBuf;
use std::result::Result;

use dialoguer::Confirm;
use dialoguer::Password;
use structopt::StructOpt;

#[derive(StructOpt)]
// #[structopt(rename_all = "kebab-case")] // unused at the moment
struct Opts {
    domain: String,
    #[structopt(short = "c", long = "confirm", help = "Ask for password confirmation")]
    confirm: bool,
    #[structopt(
        short = "n",
        long = "no-newline",
        help = "Print only the password without linebreak"
    )]
    no_newline: bool,
    #[structopt(
        short = "u",
        long = "user",
        help = "Add a username",
        default_value = ""
    )]
    username: String,
}

const PASSWORD: Encoding = new_encoding! {
    symbols: "ABCDEF$!:+-.GHJKLMNPQRSTUVWXYZabcdefghijknopqrstuvwxyz0123456789",
    // use those symbols to force multiple iterations. The only common special
    // character is $
    // symbols: "_)([]$ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijknopqrstuvwxyz0123456789",
};

const HASH_SIZE: usize = 32;

fn main() {
    let exit_code = match get_home_dir().and_then(with_home).and_then(with_hardkey) {
        Ok(_) => 0,
        Err(err) => {
            println!("{}", err);
            1
        }
    };
    std::process::exit(exit_code);
}

fn get_home_dir() -> Result<PathBuf, &'static str> {
    match home_dir() {
        None => Result::Err("Impossible to get your home dir!"),
        Some(path) => Ok(path),
    }
}

fn with_home(path: std::path::PathBuf) -> Result<String, &'static str> {
    println!("Your home directory: {}", path.display());
    let keyfile: PathBuf = path.join(".nobrain");

    match fs::read_to_string(&keyfile) {
        Ok(hardkey) => Ok(hardkey),
        Err(_) => {
            println!("Hard key file does not exist");
            let do_create = Confirm::new()
                .with_prompt("Would you like to create a .nobrain file in your home directory ?")
                .interact()
                .unwrap();
            if do_create {
                let hardkey = "__TEST__TEST__TEST__";
                match fs::write(keyfile, hardkey) {
                    Ok(_) => Ok(String::from(hardkey)),
                    _ => Err("Could not write .nobrain file"),
                }
            } else {
                Err("Canceled")
            }
        }
    }
}

fn with_hardkey(hardkey: String) -> Result<bool, &'static str> {
    let args = Opts::from_args();
    let master_key = get_master_key(args.confirm);
    let hash_data = if args.username.is_empty() {
        [hardkey, args.domain.to_string()].join("")
    } else {
        [hardkey, args.username.to_string(), args.domain.to_string()].join("")
    };
    let mut iterations: i32 = 0;
    let result = create_password(hash_data, &master_key, &mut iterations);
    if args.no_newline {
        print!("{}", result);
    } else {
        println!("Domain         {} ", args.domain);
        if !args.username.is_empty() {
            println!("Username       {} ", args.username);
        }
        if iterations > 1 {
            println!("Iterations     {} ", iterations);
        }
        // println!("hash:          {} ", hash);
        println!("Your password  {} ", result);
    };
    Ok(true)
}

fn get_master_key(confirm: bool) -> String {
    if confirm {
        Password::new()
            .with_prompt("Master key")
            .with_confirmation("Confirm master key", "The keys don't match.")
            .interact()
            .unwrap()
    } else {
        Password::new()
            .with_prompt("Master key")
            .interact()
            .unwrap()
    }
}

fn create_password(hashable: String, master_key: &String, iter: &mut i32) -> String {
    create_password_2(hashable.as_bytes(), master_key.as_bytes(), iter)
}

fn create_password_2(hashable: &[u8], master_key: &[u8], iter: &mut i32) -> String {
    *iter += 1;
    let mut hash = [0u8; HASH_SIZE];
    pbkdf2::<Hmac<Sha256>>(hashable, master_key, HASH_SIZE, &mut hash);
    let pwd = PASSWORD.encode(&mut hash);
    // As long as we do not have all features we re-encode the result
    // recursively
    if has_all_features(&pwd) {
        pwd
    } else if *iter > 100 {
        panic!("Could not determine a proper password");
    } else {
        create_password_2(pwd.as_bytes(), master_key, iter)
    }
}

lazy_static! {
    static ref RE_SMALLS: Regex = Regex::new(r"[a-z]").unwrap();
    static ref RE_CAPS: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref RE_DIGITS: Regex = Regex::new(r"[0-9]").unwrap();
    static ref RE_SPECIALS: Regex = Regex::new(r"(\$|!|\+|\-|\.|:)").unwrap();
}

// required features:
// - at least on capital letter
// - at least on small letter
// - at least a digit
// - at least special char
fn has_all_features(str: &String) -> bool {
    RE_SMALLS.is_match(str)
        && RE_CAPS.is_match(str)
        && RE_DIGITS.is_match(str)
        && RE_SPECIALS.is_match(str)
}
