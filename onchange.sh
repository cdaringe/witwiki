# rm -f wit.db && cargo watch -s 'bash onchange.sh'
set -eo pipefail
export RUST_BACKTRACE=1
export DATABASE_URL=sqlite://wit.db?mode=rwc

projects=("witwiki_common" "witwiki_db" "witwiki_migrate" "witwiki_server")

for project in "${projects[@]}"; do
  cargo fmt
  case $project in
    witwiki_migrate | witwiki_server)
      echo ".: running project $project"
      cargo build -p $project
      cargo run -p $project;;
    *)
      echo ".: building project $project"
      cargo build -p $project;;
  esac
done
