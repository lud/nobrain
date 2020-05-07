#[macro_use]
extern crate data_encoding_macro;
use data_encoding::Encoding;
use hmac::Hmac;
use lazy_static::lazy_static;
use pbkdf2::pbkdf2;
use regex::Regex;
use sha2::Sha256;

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
    let args = Opts::from_args();
    let master_key = get_master_key(args.confirm);
    let hash_data = if args.username.is_empty() {
        args.domain.to_string()
    } else {
        [args.username.to_string(), args.domain.to_string()].join("")
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
    }
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
