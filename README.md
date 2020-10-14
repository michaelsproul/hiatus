Hiatus
======

Hiatus is a concurrency debugging library for Rust. It allows you to sprinkle breakpoints
in your programs so that blocks of code execute in the order you choose. If you suspect that a
specific interleaving of blocks is buggy, you can use Hiatus to invoke that ordering and
confirm the existence of the bug.

## Motivation

Drunk on the promise of fearless concurrency you have adorned your program with type-safe
talismans — `Arc`, `Mutex` and `RwLock` feel like home now. Your program executes rapidly
on dozens of threads, without a care in the world for the data races that plagued the 20th
century. You sleep easy in your bed knowing that a ragtag gang of millennial researchers
have vetted the `unsafe` blocks upon which your fortune rests. And yet... something
doesn't feel right. Your program's plethora of locks, interleaved with filesystem writes,
has become disorderly. High-level invariants about the consistency of your different data
structures become nigh impossible to maintain as the number of locks and threads climbs
feverishly higher, higher. A user on Discord reports a catastrophic crash that no sane and
loving language designer could ever allow to happen. It seems to relate to a very specific
ordering of events in your program, one that you had never considered before. Before your
dismay deepens, a quartet of levitating neo-soul musicians appear to you a dream, granting
you the ability to see through walls~

## Solution

See the [examples](./examples) directory for examples of using Hiatus to control concurrent
execution.

## License

Apache 2.0
