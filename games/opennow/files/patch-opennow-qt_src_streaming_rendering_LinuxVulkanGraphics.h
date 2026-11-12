--- opennow-qt/src/streaming/rendering/LinuxVulkanGraphics.h.orig
+++ opennow-qt/src/streaming/rendering/LinuxVulkanGraphics.h
@@ -4,7 +4,7 @@
 #include <QtGlobal>
 #include <QtGui/qtguiglobal.h>
 
-#if defined(Q_OS_LINUX) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
 #include <QVulkanInstance>
 #include <QVulkanFunctions>
 #include <QSGRendererInterface>
