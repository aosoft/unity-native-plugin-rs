#!/bin/sh

bindgen \
  --output ./src/plugin_api_vulkan.rs \
  --with-derive-default \
  --no-derive-debug \
  --allowlist-file ./include/IUnityGraphicsVulkan.h \
  --blocklist-file ./include/IUnityInterface.h \
  --blocklist-file ./include/IUnityGraphics.h \
  --blocklist-file ./include/IUnityRenderingExtensions.h \
  ./wrappers/vulkan.hpp -- -I ./include -I $VULKAN_SDK/Include
sed -i.bak -e "s/extern \"C\"/extern \"system\"/g" ./src/plugin_api_vulkan.rs && rm ./src/plugin_api_vulkan.rs.bak
