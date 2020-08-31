#!/bin/sh

bindgen \
  --output ./src/plugin_api.rs \
  --with-derive-default \
  --no-derive-debug \
  wrapper.hpp -- -I ./include
sed -i -e "s/extern \""C\""/extern \""system\""/g" ./src/plugin_api.rs
