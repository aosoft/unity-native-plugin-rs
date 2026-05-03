#!/bin/sh

bindgen \
  --output ./src/plugin_api_windows.rs \
  --with-derive-default \
  --no-derive-debug \
  --allowlist-file ./include/IUnityGraphicsD3D11.h \
  --allowlist-file ./include/IUnityGraphicsD3D12.h \
  --blocklist-file ./include/IUnityInterface.h \
  --blocklist-file ./include/IUnityGraphics.h \
  --blocklist-file ./include/IUnityRenderingExtensions.h \
  ./wrappers/windows.hpp -- -I ./include
sed -i.bak -e "s/extern \"C\"/extern \"system\"/g" ./src/plugin_api_windows.rs && rm ./src/plugin_api_windows.rs.bak
