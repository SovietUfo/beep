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


fn main() {
    
}
