use std::process::Command;
use xtask_wasm::{anyhow::Result, clap};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    match clap::Parser::parse() {
        Opt::Dist(dist) => {
            dist.assets_dir("assets").app_name("hello").build("hello")?;
        }
        Opt::Watch(watch) => {
            let mut command = Command::new("cargo");
            command.args(["check", "--target", "wasm32-unknown-unknown"]);
            watch.run(command)?;
        }
        Opt::Start(dev_server) => {
            dev_server.xtask("dist").start()?;
        }
    }

    Ok(())
}
