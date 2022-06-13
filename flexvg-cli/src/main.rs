use clap::Parser;
use flexvg::compute_svg_string;
use log::debug;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Parser)]
#[clap(name = "flexvg")]
#[clap(author, version, about = "Create SVG diagrams with flexbox", long_about = None)]
struct Cli {
    #[clap(parse(from_os_str))]
    input: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    output: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();
    dotenv::dotenv().ok();
    let base = args
        .input
        .parent()
        .ok_or_else(|| anyhow::format_err!("No base path"))?;
    let input = File::open(&args.input)?;
    let buf_reader = BufReader::new(input);
    let root = match args
        .input
        .extension()
        .ok_or_else(|| anyhow::format_err!("expected extension"))?
        .to_str()
        .ok_or_else(|| anyhow::format_err!("expected extension as str"))?
    {
        "yaml" | "yml" => serde_yaml::from_reader(buf_reader)?,
        "json" => serde_json::from_reader(buf_reader)?,
        _ => anyhow::bail!(
            "unsupported extension, input file must be yaml or json"
        ),
    };
    debug!("{:?}", root);
    let svg = compute_svg_string(root, Some(base))?;
    let output = args.output.unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.set_extension("svg");
        path
    });
    std::fs::write(&output, svg)?;
    Ok(())
}
