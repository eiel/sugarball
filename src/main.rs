use std::process::Command;
use std::str::FromStr;
use std::string::String;

fn main() {
    println!("{:?}", installed_list());
}

fn installed_list() -> Vec<String> {
    let output = Command::new("brew")
        .arg("list")
        .arg("--full-name")
        .output()
        .expect("failed to execute process");
    let s = String::from_utf8(output.stdout).expect("no utf8");
    s.split('\n')
        .filter(|x| *x != "")
        .map(|s| String::from_str(s).expect(""))
        .collect()
}
