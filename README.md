# Introducing `wifipass`

Interestingly, Windows saves Wi-Fi passwords in plaintext. The `wifipass` command-line utility is thus a simple Wi-Fi password extractor for Windows written in Rust. 

> [!NOTE]
> One could technically use the `netsh` command to achieve the exact same thing; `wifipass` is just a simpler shorthand.

> [!CAUTION]
> As `wifipass` deals with highly sensitive Wi-Fi passwords, it is worth reiterating that this project and its contributors shall not be held liable for damages and leaked information. See the [MIT license](./LICENSE) for more details.

## Development

Until the [`try_find` feature](https://github.com/rust-lang/rust/issues/63178) is stable, `wifipass` requires a nightly Rust compiler.

```bash
# Run `wifipass` with optimizations enabled.
cargo run --release
```

## Special Thanks

This project is a rewrite of [John Hammond's](https://www.youtube.com/@_JohnHammond) [original implementation](https://www.youtube.com/watch?v=auGJJOfmrMM) in Rust. The `wifipass` codebase improves on this by better leveraging "Rusty" patterns and idioms (that were missed in the video).
