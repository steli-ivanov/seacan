pub mod config;
pub mod constants;
pub mod log;

async fn application() -> anyhow::Result<()> {
    outputln!("started");

    Ok(())
}

fn main() {
    if let Err(e) = config::load() {
        errorln!("failed to initialize the config, error: {}", e.to_string());
        std::process::exit(1);
    }

    log::initialize();

    let config = config::object();
    let worker_threads = if config.number_of_workers == 0 {
        std::thread::available_parallelism()
            .expect("failed to get the number of cores")
            .get()
    } else {
        config.number_of_workers
    };

    match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(worker_threads)
        .build()
    {
        Ok(runtime) => {
            runtime.block_on(async {
                if let Err(e) = application().await {
                    errorln!("runtime error: {}", e.to_string());
                    std::process::exit(1);
                }
            });
        }
        Err(e) => {
            errorln!(
                "failed to build the async runtime, error: {}",
                e.to_string()
            );
            std::process::exit(1);
        }
    }
}
