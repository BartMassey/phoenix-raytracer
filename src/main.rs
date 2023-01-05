use std::path::PathBuf;

use rt::*;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, default_value="render.out.ppm")]
    outfile: PathBuf,
    #[arg(required(true))]
    x: usize,
    #[arg(required(true))]
    y: usize,
}

fn main() {
    let args = Args::parse();

    let scene = Thing::generate();
    let m = Model::generate(scene);

    let out = PpmRawOutput::new(&args.outfile, args.x, args.y).unwrap();

    render(out, &m, args.x, args.y);
}
