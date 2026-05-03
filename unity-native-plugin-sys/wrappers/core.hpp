#include <cstdint>
#include <cstddef>

#define UINT32 uint32_t
#define UINT uint32_t

#include <IUnityInterface.h>
#include <IUnityGraphics.h>
#include <IUnityRenderingExtensions.h>

#ifndef NULL
#define NULL nullptr
#endif

#include <IUnityProfiler.h>
#include <IUnityProfilerCallbacks.h>

#include <IUnityMemoryManager.h>

#define Assert(x)

#include <IUnityEventQueue.h>
#include <IUnityLog.h>
#include <IUnityShaderCompilerAccess.h>
