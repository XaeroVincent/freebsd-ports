--- opennow-qt/src/streaming/rendering/LinuxVulkanGraphics.cpp.orig
+++ opennow-qt/src/streaming/rendering/LinuxVulkanGraphics.cpp
@@ -1,6 +1,6 @@
 #include "streaming/rendering/LinuxVulkanGraphics.h"
 
-#if defined(Q_OS_LINUX) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
 #include <QGuiApplication>
 #include <QPlatformSurfaceEvent>
 #include <QQuickGraphicsDevice>
