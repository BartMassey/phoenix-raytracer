# Phoenix Raytracer
Copyright (c) 2017 Bart Massey

Long ago (1991) I wrote a raytracer in C++
<http://github.com/BartMassey/ancient-raytracer> for a
graduate course (Gary Meyer's Computer Graphics at
U. Oregon). The exercise confirmed my dislike of C++ and
took 20 years and some help to fully debug, but the
resulting raytracer makes a pretty picture.

This repo represents my "port" of that raytracer to Rust.
This isn't a particularly literal translation: I tried to
use best practices in Rust and SE rather than preserve the
details. The architecture is similar, and the algorithms are
the same. (Thus, it's really more like a rebirth from the
ashes---hence the name.)

## Issues

* I really didn't want to mess with Rust's insistence on
  thread-safe PRNG operation, and I didn't need a
  particularly good generator. So I used a 64-bit LCG with
  unsafe global state.

## License

This program is licensed under the "3-clause ('new') BSD
License". Please see the file COPYING in this distribution
for license terms.
