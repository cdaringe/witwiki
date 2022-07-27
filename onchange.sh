# rm -f wit.db && cargo watch -s 'bash onchange.sh'
set -eo pipefail

projects=("witwiki_common" "witwiki_db" "witwiki_migrate" "witwiki_server")

for project in "${projects[@]}"; do
  cargo fmt
  case $project in
    witwiki_migrate | witwiki_server)
      echo ".: running project $project"
      cargo run -p $project;;
    *)
      echo ".: building project $project"
      cargo build -p $project;;
  esac
done