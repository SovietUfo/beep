use std::{fs::OpenOptions, os::unix::prelude::AsRawFd, thread, time::Duration};
use nix::ioctl_write_int_bad;
use structopt::StructOpt;


const KIOCSOUND: u64 = 0x4B2F;
// genuinely no idea where this comes from but it makes things work
const CLOCK_TICK_RATE: u32 = 1193180;

#[derive(StructOpt, Debug)]
struct Opt {
    /// frequency of the beep (in hz)
    #[structopt(short, long)]
    freq: u16,
    #[structopt(short, long)]
    /// how long should the beep be (in ms)
    time: u32
}

ioctl_write_int_bad!(kiocsound, KIOCSOUND);

fn main() {
    let args = Opt::from_args();

    let device = OpenOptions::new().append(true).open("/dev/console").expect("couldn't open /dev/console");
    let period = CLOCK_TICK_RATE.checked_div(args.freq as u32).unwrap_or(0);

    unsafe { kiocsound(device.as_raw_fd(), period as i32).unwrap(); }
    thread::sleep(Duration::from_millis(args.time as u64));
    unsafe { kiocsound(device.as_raw_fd(), 0).unwrap(); }
}
