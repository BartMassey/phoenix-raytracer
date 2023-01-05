use std::path::PathBuf;

use rt::*;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value = "render.out.ppm")]
    outfile: PathBuf,
    #[arg(short, long)]
    sequential: bool,
    #[arg(required(true))]
    width: usize,
    #[arg(required(true))]
    height: usize,
}

fn main() {
    let args = Args::parse();

    let scene = Thing::generate();
    let m = Model::generate(scene);

    let out = PpmRawOutput::new(&args.outfile, args.width, args.height).unwrap();

    render(out, &m, args.width, args.height, args.sequential);
}
