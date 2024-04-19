use xshell::{cmd, Shell};

mod chroot;
mod error;
mod kernel;
mod kit;
mod rootfs;

fn main() {
    println!("Hello, world!");

    let sh = Shell::new().unwrap();

    let size = kit::report_size(&sh, "srcf");
    println!("{:?}", size.err());
}
