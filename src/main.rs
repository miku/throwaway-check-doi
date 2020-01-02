#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

use failure::Error;
use regex::Regex;
use std::result;

/// Result is a a type alias for handling errors (throughout this crate).
type Result<T> = result::Result<T, Error>;

/// check_doi validates a DOI.
fn check_doi(raw: &str) -> Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^10.\d{3,6}/\S+$").unwrap();
    }
    if raw.is_ascii() && RE.is_match(raw) {
        Ok(())
    } else {
        let msg = format!(
            "DOI wanted, e.g. '10.1234/aksjdfh', got {}",
            raw.to_string()
        );
        Err(format_err!("{}", msg))
    }
}

fn main() {
    let vec = vec![
        "Hello",
        "World",
        "10.1234/aksjdfh abz",
        "10.1234/aksjdfh",
        "10.25513/1812-3996.2017.1.34–42", // 8211, Hex 2013, Octal 20023
        "10.25513/1812-3996.2017.1.34",
        "10.25513/1812-3996.2017.1.34-",
        "10.25513/1812-3996.2017.1.34-42", // 45, Hex 2d, Octal 055
    ];

    // The problem is a ok looking dash, which falls into the EN QUAD..HAIR SPACE range
    // (https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt).
    //
    // 2000..200A    ; White_Space # Zs  [11] EN QUAD..HAIR SPACE
    for v in &vec {
        let _ = match check_doi(v) {
            Ok(_) => println!("{} => ok", *v),
            Err(_) => println!("{} => fail", *v),
        };
    }
}
