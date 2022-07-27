# witwiki

[![Rust](https://github.com/cdaringe/witwiki/actions/workflows/main.yml/badge.svg)](https://github.com/cdaringe/witwiki/actions/workflows/main.yml)

A practical wiki, for persons.

## install

1. **Option 1**: `cargo install witwiki` (tbd)

## run

### dev

- `rm -f wit.db* && cargo watch -s 'bash onchange.sh'`

### prod

- set `RUST_ENV=production`

## why

The vast majority of wiki softwares:

- require a bloated or complex software stack deployed in tandem
- use less-than-safe languages (php, java, et all)

Witiki is _small_, sane, and portable.
