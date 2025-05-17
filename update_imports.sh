#!/bin/zsh
find /Users/david/Desktop/postgre-rust/src -type f -name "*.rs" | xargs sed -i '' 's/use crate::/use postgre_rust::/g'
