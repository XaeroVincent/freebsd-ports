--- opennow-qt/src/streaming/rendering/NativeStreamRenderCallback.cpp.orig
+++ opennow-qt/src/streaming/rendering/NativeStreamRenderCallback.cpp
@@ -109,7 +109,7 @@
                 context.device = reinterpret_cast<void *>(handles->dev);
                 context.queue = reinterpret_cast<void *>(handles->gfxQueue);
                 context.queue_family_index = handles->gfxQueueFamilyIdx;
-#if defined(Q_OS_LINUX)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD))
                 if (m_runtime && m_runtime->vulkanDevice()) {
                     OpenNowStreamerVulkanDeviceInfo info{};
                     info.version = OPENNOW_STREAMER_VULKAN_DEVICE_INFO_VERSION;
