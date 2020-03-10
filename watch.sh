#!/bin/sh

cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build --debug --target web"