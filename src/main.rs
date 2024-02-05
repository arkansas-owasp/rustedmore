fn main() {
    println!("Hello, world!");
    println!("Hello, world!");
    // println!("'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'");
    println!(r"'X5O!P%@AP[4\PZX54(P^)7CC)7}}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'");
    println!("Yet another change but it is just the 4th println statement");

}
// adding comments to see if it yields a new hash 20240202Fri1627 AL
// adding comments did NOT change the hash so I added the hello world function a 2nd time
// added the EICAR test string 20240202Fri1708 X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*
// bracketed the EICAR string with single quotes since I got an odd Unrecognized escape error
// println!("'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'");
// After 2 failures I read the Rust help provided w/the error and did this 
// println!(r"'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'");
// yet another error message involving weird escapes and literals
// error: invalid format string: unmatched `}` found
// --> src/main.rs:5:42
//  |
// 5 |     println!(r"X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*");
//  |                                          ^ unmatched `}` in format string
//  |
//  = note: if you intended to print `}`, you can escape it using `}}`
// trying this instead println!(r"'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'");
// no dice trying this: println!(r"'X5O!P%@AP[4\PZX54(P^)7CC)7}}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'"); (adding a second } to escape the first one
