#!/bin/sh

bindgen \
  --output ./src/metal_api.rs \
  --with-derive-default \
  --no-derive-debug \
  --blocklist-type "NSBundle|MTLRenderPassDescriptor|INSBundle|IMTLRenderPassDescriptor" \
  --raw-line "pub type NSBundle = *mut u8;" \
  --raw-line "pub type MTLRenderPassDescriptor = *mut u8;" \
  wrapper.hpp -- -x objective-c++ -I ./include -I $VULKAN_SDK/Include
sed -i -e "s/extern \"C\"/extern \"system\"/g" ./src/metal_api.rs
