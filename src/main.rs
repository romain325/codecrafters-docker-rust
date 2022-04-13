pub mod container_file_sys;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    match container_file_sys::init_fs(command.to_string()) {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{:?}", err);
            std::process::exit(1);
        }
    };

    let output = std::process::Command::new(command)
        .args(command_args)
        .output()
        .unwrap();

    let std_out = std::str::from_utf8(&output.stdout).unwrap();
    let std_err = std::str::from_utf8(&output.stderr).unwrap();
    if !std_out.is_empty() {
        print!("{}", std_out);
    }
    if !std_err.is_empty() {
        eprint!("{}", std_err);
    }

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap());
    }
}
