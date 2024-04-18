use xshell::{cmd, Shell};

mod chroot;
mod kernel;
mod kit;
mod rootfs;

fn main() {
    println!("Hello, world!");

    let sh = Shell::new().unwrap();

    let size = kit::report_size(&sh, "src");
    println!("{:?}", size);
}
