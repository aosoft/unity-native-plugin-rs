#!/bin/sh

bindgen \
  --output ./src/plugin_api_core.rs \
  --with-derive-default \
  --no-derive-debug \
  --allowlist-file ./include/IUnityInterface.h \
  --allowlist-file ./include/IUnityGraphics.h \
  --allowlist-file ./include/IUnityRenderingExtensions.h \
  --allowlist-file ./include/IUnityProfiler.h \
  --allowlist-file ./include/IUnityProfilerCallbacks.h \
  --allowlist-file ./include/IUnityMemoryManager.h \
  --allowlist-file ./include/IUnityEventQueue.h \
  --allowlist-file ./include/IUnityLog.h \
  --allowlist-file ./include/IUnityShaderCompilerAccess.h \
  ./wrappers/core.hpp -- -I ./include
sed -i.bak -e "s/extern \"C\"/extern \"system\"/g" ./src/plugin_api_core.rs && rm ./src/plugin_api_core.rs.bak
