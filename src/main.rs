pub mod searchable;
pub mod ssh;
pub mod ssh_config;
pub mod ui;

use anyhow::{Result, Context};
use clap::Parser;
use reqwest::blocking::get;
use std::fs;
use std::io::Write;
use std::path::Path;
use ui::{App, AppConfig};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to download the SSH configuration file from
    #[arg(short, long)]
    url: Option<String>,

    /// Path to the SSH configuration file
    #[arg(
        short,
        long,
        num_args = 1..,
        default_values_t = [ "~/.ssh/wisperServers".to_string(), ]
    )]
    config: Vec<String>,

    /// Shows `ProxyCommand`
    #[arg(long, default_value_t = false)]
    show_proxy_command: bool,

    /// Host search filter
    #[arg(short, long)]
    search: Option<String>,

    /// Sort hosts by hostname
    #[arg(long, default_value_t = true)]
    sort: bool,

    /// Handlebars template of the command to execute
    #[arg(short, long, default_value = "ssh -t SECURED_BASTION \"{{user}}\"@\"{{{destination}}}\"")]
    template: String,

    /// Exit after ending the SSH session
    #[arg(short, long, default_value_t = false)]
    exit: bool,
}

fn download_file(url: &str, destination: &Path) -> Result<()> {
    let response = get(url).context("Failed to download file")?;
    let mut file = fs::File::create(destination).context("Failed to create file")?;
    let content = response.bytes().context("Failed to read response bytes")?;
    file.write_all(&content).context("Failed to write to file")?;
    Ok(())
}


fn main() -> Result<()> {
    let args = Args::parse();

    // Define the local file path
    let home_dir = dirs::home_dir().context("Could not find home directory")?;
    let local_file_path = home_dir.join(".ssh").join("wisperServers");

    // Check if the URL argument is provided and download the file if it does not exist
    if let Some(url) = &args.url {
        download_file(url, &local_file_path).context("Failed to download and save SSH configuration file")?;
    }
    // Continue with the existing logic
    let mut app = App::new(&AppConfig {
        config_paths: args.config,
        search_filter: args.search,
        sort_by_name: args.sort,
        show_proxy_command: args.show_proxy_command,
        command_template: args.template,
        exit_after_ssh: args.exit,
    })?;
    app.start()?;

    Ok(())
}
