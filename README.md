# Introducing `wifipass`

Interestingly, Windows saves Wi-Fi passwords in plaintext. The `wifipass` command-line utility is thus a simple Wi-Fi password extractor for Windows written in Rust. 

> [!NOTE]
> One could technically use the `netsh` command to achieve the exact same thing; `wifipass` is just a simpler shorthand.

## Special Thanks

This project is a rewrite of [John Hammond's](https://www.youtube.com/@_JohnHammond) [original implementation](https://www.youtube.com/watch?v=auGJJOfmrMM) in Rust. The `wifipass` codebase improves on this by better leveraging "Rusty" patterns and idioms (that were missed in the video).
