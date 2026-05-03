#ifndef UNITY_NATIVE_PLUGIN_RS_VULKAN_STUBS_H
#define UNITY_NATIVE_PLUGIN_RS_VULKAN_STUBS_H

// Minimal Vulkan type stubs to avoid a build-time dependency on the Vulkan SDK.
// These match the Vulkan ABI for 64-bit targets. See LICENSE_VulkanHeaders.md.

#include <stdint.h>

typedef uint32_t VkFlags;
typedef uint64_t VkDeviceSize;

// Dispatchable handles
typedef struct VkInstance_T*        VkInstance;
typedef struct VkPhysicalDevice_T*  VkPhysicalDevice;
typedef struct VkDevice_T*          VkDevice;
typedef struct VkQueue_T*           VkQueue;
typedef struct VkCommandBuffer_T*   VkCommandBuffer;

// Non-dispatchable handles (pointer-sized on 64-bit per Vulkan spec)
typedef struct VkPipelineCache_T*   VkPipelineCache;
typedef struct VkDeviceMemory_T*    VkDeviceMemory;
typedef struct VkImage_T*           VkImage;
typedef struct VkBuffer_T*          VkBuffer;
typedef struct VkRenderPass_T*      VkRenderPass;
typedef struct VkFramebuffer_T*     VkFramebuffer;

// Flag aliases (all VkFlags-backed in the Vulkan spec)
typedef VkFlags VkMemoryPropertyFlags;
typedef VkFlags VkImageAspectFlags;
typedef VkFlags VkImageUsageFlags;
typedef VkFlags VkBufferUsageFlags;
typedef VkFlags VkPipelineStageFlags;
typedef VkFlags VkAccessFlags;

// Enum types (sized as int per C ABI; concrete values are handled at the ash layer)
typedef unsigned int VkImageLayout;
typedef unsigned int VkFormat;
typedef unsigned int VkImageTiling;
typedef unsigned int VkImageType;
typedef unsigned int VkSampleCountFlagBits;
typedef unsigned int VkCommandBufferLevel;

// Value-embedded struct
typedef struct VkExtent3D {
    uint32_t width;
    uint32_t height;
    uint32_t depth;
} VkExtent3D;

// Pointer-only forward declaration
typedef struct VkImageSubresource VkImageSubresource;

// Function pointer typedefs
typedef void (*PFN_vkVoidFunction)(void);
typedef PFN_vkVoidFunction (*PFN_vkGetInstanceProcAddr)(VkInstance instance, const char* pName);

#endif
