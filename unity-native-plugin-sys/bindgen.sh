#!/bin/sh

bindgen \
  --output ./src/plugin_api.rs \
  --with-derive-default \
  --with-derive-eq \
  wrapper.hpp -- -I ./include
