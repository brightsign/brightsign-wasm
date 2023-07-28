use std::process::Command;
use xtask_wasm::{anyhow::Result, clap, default_dist_dir};

#[derive(clap::Parser)]
enum Opt {
    Dist(xtask_wasm::Dist),
    Watch(xtask_wasm::Watch),
    Start(xtask_wasm::DevServer),
}

fn main() -> Result<()> {
    let opt: Opt = clap::Parser::parse();

    match opt {
        Opt::Dist(dist) => {
            println!("Generating package");
            dist
                .dist_dir_path("dist")
                .static_dir_path("brightsign-wasm/static")
                .app_name("brightsign-wasm")
                .run_in_workspace(true)
                .run("brightsign-wasm").expect("Failed to generate package");
        }
        Opt::Watch(watch) => {
            println!("Watching for changes");
            let mut command = Command::new("cargo");

            command.arg("check");

            watch.run(command).expect("Failed to run the cargo check command");
        }
        Opt::Start(mut dev_server) => {
            println!("Starting the dev server");
            dev_server.arg("dist").start(default_dist_dir(true)).expect("Failed to start the dev server");
        }
    }

    Ok(())
}
