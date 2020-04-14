# FRACTRAN

A rust implementation of [FRACTRAN](https://en.wikipedia.org/wiki/FRACTRAN),
thanks to [John Conway](https://en.wikipedia.org/wiki/John_Horton_Conway).

----

Reads programs from stdin, first comma separated fractions of the form `<int>/<int>`, then a final comma and then the starting state. Output goes to stdout, one line per iteration.

----

The easiest way to run is something like `cargo run < add.frac`. That will add 1 and 2.

---
`add.frac` is about the smallest possible program. Wikipedia does a better job explaining it than I ever will, so just note the last item in that file is 18 which is `2^1 * 3^2`. When you run it the final output
is 27, aka `3^(1 + 2)`.

----

#### status
`add.frac` certainly works, but I'm not sure about the `primes.frac`. It is entirely possible everything is fine and there is a transcription error, it is also highly probable there is a horrible mistake somewhere.

This is also the first time I've ever written any rust, so don't be too upset. Just lie back and think about the Collatz conjecture.
