use std::path::Path;
use std::ffi::CString;
use std::fs;
use std::io;

pub fn init_fs(command: String) -> io::Result<String> {
    let dir = create_dir(command.clone())?;
    let new_path = copy_bin(&mut dir.clone(), command)?;
    
    unsafe {
        let val = CString::new(dir).unwrap().into_raw() as *const libc::c_char;
        let current = CString::new(std::env::current_dir()?.to_str().unwrap()).unwrap().into_raw() as *const libc::c_char;
        //println!("{}",libc::chroot(val));
        libc::chroot(val);
        libc::unshare(libc::CLONE_NEWPID);
        // if libc::syscall(libc::SYS_pivot_root, val, current) == -1 {
        //     println!("{}", io::Error::last_os_error());
        // }
    }
    std::env::set_current_dir("/")?;
    Ok(new_path)
}

fn create_dir(command: String) -> io::Result<String> {
    let dir = String::from("/app/sandbox");
    let mut bin_dir = dir.clone();
    bin_dir.push_str(&command);

    if Path::new(&dir).exists() {
        fs::remove_dir_all(dir.clone())?;
    }

    fs::create_dir_all(Path::new(&bin_dir).parent().unwrap())?;
    fs::create_dir(format!("{}/dev", dir))?;
    fs::File::create(format!("{}/dev/null",dir))?;

    Ok(dir)
}

fn copy_bin(pre: &mut String, command : String) -> io::Result<String> {
    pre.push_str(&command);
    fs::copy(command, pre.clone())?;
    Ok(pre.to_string())
}