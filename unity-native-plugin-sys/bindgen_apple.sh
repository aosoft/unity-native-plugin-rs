#!/bin/sh

bindgen \
  --output ./src/plugin_api_apple.rs \
  --with-derive-default \
  --no-derive-debug \
  --allowlist-file ./include/IUnityGraphicsMetal.h \
  --blocklist-file ./include/IUnityInterface.h \
  --blocklist-file ./include/IUnityGraphics.h \
  --blocklist-file ./include/IUnityRenderingExtensions.h \
  --blocklist-type "NSBundle|MTLRenderPassDescriptor|INSBundle|IMTLRenderPassDescriptor" \
  --raw-line "pub type NSBundle = *mut u8;" \
  --raw-line "pub type MTLRenderPassDescriptor = *mut u8;" \
  ./wrappers/apple.hpp -- -x objective-c++ -I ./include
sed -i.bak -e "s/extern \"C\"/extern \"system\"/g" ./src/plugin_api_apple.rs && rm ./src/plugin_api_apple.rs.bak
