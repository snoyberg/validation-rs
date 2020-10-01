# validation

![Continuous integration](https://github.com/snoyberg/validation-rs/workflows/Continuous%20integration/badge.svg)

Provides a `Validation` enum. `Validation` provides a `FromIterator` implementation which is just like `Result`'s, except it will collect multiple error values instead of just the first.

Performance can probably be improved, and documentation is basically nonexistent. If there's interest in making this production grade, let me know in the issue tracker!