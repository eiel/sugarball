extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::process::Command;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
struct Formula {
    full_name: String,
    linked_keg: Option<String>,
}

fn main() {
    let c = installed_list();
    println!("{:?}", c);
}

fn get_installed_info_command() -> Command {
    let mut cmd = Command::new("brew");
    cmd
        .arg("info")
        .arg("--json=v1")
        .arg("--installed");
    cmd
}

fn installed_list() -> Vec<Formula> {
    let output = get_installed_info_command()
        .output()
        .expect("failed to execute process");
    let s = String::from_utf8(output.stdout).expect("no utf8");
    serde_json::from_str(&s.into_boxed_str()).expect("hoge")
}
