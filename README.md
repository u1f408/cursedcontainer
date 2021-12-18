[![Crates.io](https://img.shields.io/crates/v/cursedcontainer.svg)](https://crates.io/crates/cursedcontainer)
[![Workflow Status](https://github.com/u1f408/cursedcontainer/workflows/main/badge.svg)](https://github.com/u1f408/cursedcontainer/actions?query=workflow%3A%22main%22)

# cursedcontainer

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

## License

cursedcontainer is licensed under the MIT license,
the text of which can be found in [the LICENSE file](LICENSE),
or at <https://opensource.org/licenses/MIT>.
