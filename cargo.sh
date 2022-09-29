#!/usr/bin/env bash
set -eo pipefail
export RUST_BACKTRACE=1
export DATABASE_URL=sqlite://wit.db?mode=rwc
cargo "$@"
