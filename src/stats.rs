use std::env;
use std::error::Error;
use ansi_term::Colour::Red;

pub fn initialize() -> Result<(), Box<dyn Error>> {
    let directory = env::current_dir()?;
    log::info!("[Path] Running in {}", directory.display());

    let cpus = num_cpus::get();
    log::info!("[CPU] Unknown model Cores x{}", cpus);

    let os_platform = env::consts::OS;
    let os_arch = env::consts::ARCH;
    log::info!("[System] {} {}", os_platform, os_arch);

    #[cfg(target_os = "linux")] {
        if nix::unistd::Uid::current().is_root() {
            log::info!(Red.paint("[Process] Warning fosscord is running as root, this highly discouraged and might expose your system vulnerable to attackers. Please run fosscord as a user without root privileges."));
        }
    }

    Ok(())
}