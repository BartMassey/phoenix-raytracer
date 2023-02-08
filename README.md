# Phoenix Raytracer
Bart Massey 2017—

Long ago (1991) I wrote a raytracer in C++
<http://github.com/BartMassey/ancient-raytracer> for a
graduate course (Gary Meyer's Computer Graphics at
U. Oregon). The exercise confirmed my dislike of C++ and
took 20 years and some help to fully debug, but the
resulting raytracer makes a pretty picture.

This repo represents my "port" of that raytracer to Rust.
It's a sort of rebirth from the ashes---hence the name.

This branch improves upon the code of the `classic` branch,
which was my closest approach to a strict port of the C++
code.

## Issues

* I really didn't want to mess with Rust's insistence on
  thread-safe PRNG operation, and I didn't need a
  particularly good generator. So I used a 64-bit LCG with
  unsafe global state.

* This is a pretty inefficient port of a pretty gross
  original. It could use a ton of cleanup of grossness
  inherited from `ancient-raytracer` and a ton of
  "carcinization" to be more efficient and readable.

  The core algorithm is stupidly inefficient, checking
  intersection against every object in the scene for every
  raycast. Something as simple as octtrees, or better yet
  some kind of adaptive space partitioning, might really
  speed things up a lot.

* There are several branches here:

  * The `main` branch uses external crates where appropriate:

    * `clap` for argument parsing
    * `rayon` for parallelism

    Other things should be replaced with external crates,
    notably the graphic output.

  * The `classic` branch uses no external code, as
    `ancient-raytracer` did.

  * The `nalgebra` branch replaces the built-in `Point` code
    with a newtype of `nalgebra::DVector`. This first crude
    cut is quite a bit slower, so has been set aside for
    now. The code could probably be sped up hugely by
    getting rid of the newtype and just using
    `nalgebra::DVector` as the `Point` type, but it gets a
    bit hassle-filled so not for now.

## License

This program is licensed under the "3-clause ('new') BSD
License". Please see the file COPYING in this distribution
for license terms.
