mod analyzer;
mod parser;
mod util;

use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

fn main() -> color_eyre::eyre::Result<()> {
    init_logging();
    color_eyre::install()?;

    let (source_path, _dest_path) = parse_args();

    let parsed_source = parser::from_path(source_path)?;
    analyzer::analyze(&parsed_source)?;

    // other stuff

    Ok(())
}

fn parse_args() -> (PathBuf, PathBuf) {
    let mut args = std::env::args();
    if args.len() < 3 {
        panic!("Only got {} arguments but required 3.", args.len());
    }

    // skip the program name
    args.next().unwrap();

    let source_path = PathBuf::from(args.next().unwrap());
    let dest_path = PathBuf::from(args.next().unwrap());

    (source_path, dest_path)
}

fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or(EnvFilter::builder().parse("pimpf=info").unwrap()),
        )
        .without_time()
        .pretty()
        .init();

    tracing::debug!("Debug logging enabled");
}

#[cfg(test)]
pub fn init_color_eyre() {
    use std::sync::Once;

    static SETUP_COLOR_EYRE: Once = Once::new();

    SETUP_COLOR_EYRE.call_once(|| {
        color_eyre::install().unwrap();
    });
}
