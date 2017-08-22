use std::process::Command;
use std::string::String;

fn main() {
    let c = installed_list();
    println!("{:?}", c);
}

fn get_installed_info_command() -> Box<Command> {
    let mut cmd = Command::new("brew");
    cmd
        .arg("info")
        .arg("--json=v1")
        .arg("--installed");
    Box::new(cmd)
}

fn installed_list() -> String {
    let output = get_installed_info_command()
        .output()
        .expect("failed to execute process");
    let s = String::from_utf8(output.stdout).expect("no utf8");
    s
}
