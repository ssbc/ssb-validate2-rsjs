set -eEu -o pipefail
shopt -s extdebug
IFS=$'\n\t'
trap 'onFailure $?' ERR

function onFailure() {
  echo "Unhandled script error $1 at ${BASH_SOURCE[0]}:${BASH_LINENO[0]}" >&2
  exit 1
}

if [ "$npm_config_platform" == "android" ] || [ "$npm_config_platform" == "ios" ]; then
  cd native
  mkdir -p dist
  cargo build --release
  cd ..
  cp ./target/$CARGO_BUILD_TARGET/release/libssb_validate2_rsjs.so ./native/dist/index.node
fi
