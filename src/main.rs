use rt::*;

fn main() {
    let argv: Vec<String> = std::env::args().collect();

    let x = argv[1].parse().unwrap();
    let y = argv[2].parse().unwrap();

    let scene = Thing::generate();
    let m = Model::generate(scene);

    let out = PpmRawOutput::new("render.out", x, y).unwrap();

    render(out, &m, x, y);
}
