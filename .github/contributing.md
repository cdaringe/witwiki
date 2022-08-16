# contributing

1. setup rust toolchain, `rustup ...<todo>`
   1. i'm currently on `rustc 1.63.0-nightly (420c970cb 2022-06-09)`
1. install [cargo watch](https://lib.rs/crates/cargo-watch)
1. run `./dev.sh`. this script...
   1. creates a `wit.db` sqlite instance, hydrates it with dev content
      1. updates to migrations must yield a rerun of `dev.sh`, which prunes the db
   1. sets up a watcher, and re-runs the server on change
