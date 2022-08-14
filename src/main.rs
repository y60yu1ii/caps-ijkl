pub mod kbd;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Stewart J. Park <hello@stewartjpark.com>")]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let verbose = opts.verbose;

    let kbd = loop {
        if let Ok(keyboards) = kbd::enumerator::enumerate_keyboards() {
            println!("Keyboard count: {}", keyboards.len());
            if keyboards.len() > 0 {
                break keyboards.into_iter().next().unwrap();
            }
        } else {
            panic!("Keyboards cannot be detected.");
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    };

    println!("Keyboard \"{}\" detected.", kbd.name);

    let mut handler = kbd::handler::KeyboardHandler::new(&kbd.device_path, verbose);
    handler.run_forever();
}
