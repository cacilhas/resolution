[COPYING]: https://github.com/cacilhas/resolution/blob/master/COPYING.md
[The 3-Clause BSD License]: https://opensource.org/licenses/BSD-3-Clause

# Resolution

Crate to retrieve screen resolution/dimensions on Linux and macOS.

## Rational

I have rewrite this same code so many times that it has proved its necessity
to exist.

## Install

```sh
cargo add resolution
```

## Interface

- `resolution::current_resolution() -> Result<(i32, i32), resolution::ResolutionError>`

It receives no parameter and returns current screen’s `(width, height)`.

## License

- [COPYING][]
- [The 3-Clause BSD License][]
