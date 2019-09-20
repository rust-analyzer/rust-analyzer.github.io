#!/usr/bin/env bash
set -ex

curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

wasm-pack build ./wasm-demo
pushd wasm-demo/www && npm install && npm run build && popd
mv wasm-demo/www/dist/{*.js,*.wasm} ./
bundle exec jekyll build
