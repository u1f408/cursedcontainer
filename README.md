# cursedcontainer

[![Crates.io](https://img.shields.io/crates/v/cursedcontainer.svg)](https://crates.io/crates/cursedcontainer)
[![Workflow status](https://github.com/u1f408/cursedcontainer/actions/workflows/test.yml/badge.svg)](https://github.com/u1f408/cursedcontainer/actions/workflows/test.yml)

A "cursed" container with an opaque key type, that allows for retrieving mutable references to
the objects contained within.

The `CursedContainer` is a synchronized init-on-first-use `Vec<T>` wrapper, where the objects
within the inner Vec are themselves contained within an [`UnsafeCell`], allowing for retrieval
of mutable references to those objects without a mutable reference to the `CursedContainer`
itself.

This design allows for assigning a `CursedContainer` to a `static` variable, like so:

```rust
static CONTAINER: CursedContainer<usize> = CursedContainer::new();

let key = CONTAINER.insert(69420);
assert_eq!(CONTAINER.get(key), Some(&mut 69420));
```

## Safety

Hahah, good joke.

There is some safety built into `CursedContainer` around initialization race conditions, but
accessing items within the container is unsafe by design - it allows for retrieving multiple
mutable references to the same object stored within the container.

It is the responsibility of your application code to make sure that things don't go horribly
wrong when using a `CursedContainer`.

## But... why?

This crate was developed for the author's hobby operating system project, and the lack of
safety in here is designed for that purpose.

## License

cursedcontainer is licensed under the MIT license,
the text of which can be found in [the LICENSE file](LICENSE),
or at <https://opensource.org/licenses/MIT>.
