use std::mem;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "print (realtime - monotonic time)[secs]")]
struct Cli {
    #[structopt(long = "verbose", help = "verbose flag")]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let now = SystemTime::now();

    let (tv_sec, tv_nsec) = unsafe {
        let mut timespec: libc::timespec = mem::MaybeUninit::zeroed().assume_init();
        let ret = libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut timespec);
        if ret != 0 {
            panic!("clock_gettime failed");
        }
        (timespec.tv_sec as i64, timespec.tv_nsec as u32)
    };
    let monotonic_now_duration =
        Duration::from_nanos(tv_sec as u64 * 1000_000_000 + tv_nsec as u64);

    let realtime_now_duration = now.duration_since(UNIX_EPOCH)?;

    if args.verbose {
        eprintln!(
            "monotonic now: {:>10}.{:09}s",
            monotonic_now_duration.as_secs(),
            monotonic_now_duration.subsec_nanos()
        );
        eprintln!(
            "realtime  now: {:>10}.{:09}s",
            realtime_now_duration.as_secs(),
            realtime_now_duration.subsec_nanos()
        );
    }

    let diff_duration = realtime_now_duration - monotonic_now_duration;
    println!(
        "{}.{:09}",
        diff_duration.as_secs(),
        diff_duration.subsec_nanos()
    );
    Ok(())
}
