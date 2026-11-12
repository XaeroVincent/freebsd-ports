--- opennow-qt/src/app/ApplicationStartup.cpp.orig	2026-11-11 15:50:45.000000000 -0800
+++ opennow-qt/src/app/ApplicationStartup.cpp	2026-11-12 00:22:55.315613000 -0800
@@ -133,13 +133,13 @@ static int runApplicationSession(int argc, char *argv[
     QQuickWindow::setGraphicsApi(QSGRendererInterface::Direct3D11);
 #elif defined(Q_OS_MACOS)
     QQuickWindow::setGraphicsApi(QSGRendererInterface::Metal);
-#elif defined(Q_OS_LINUX)
+#elif (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD))
     QQuickWindow::setGraphicsApi(QSGRendererInterface::Vulkan);
 #endif
     QQuickStyle::setStyle(u"Basic"_s);
 
     QGuiApplication application(argc, argv);
-#if defined(Q_OS_LINUX) && defined(NO_SOFT_RENDERING)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && defined(NO_SOFT_RENDERING)
 #if !QT_CONFIG(vulkan) || !__has_include(<vulkan/vulkan.h>)
     qCritical("NO_SOFT_RENDERING requires a Qt build with Vulkan support.");
     return EXIT_FAILURE;
@@ -153,7 +153,7 @@ static int runApplicationSession(int argc, char *argv[
     // On macOS, Cocoa must resolve the bundle's appearance-aware icon.
     QGuiApplication::setWindowIcon(applicationWindowIcon());
 #endif
-#if defined(Q_OS_LINUX) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
     LinuxVulkanGraphics::requestDeviceExtensions();
 #endif
     registerStreamVideoItemQmlType();
@@ -224,7 +224,7 @@ static int runApplicationSession(int argc, char *argv[
     );
     QObject::connect(&localization, &Localization::localeChanged,
                      &graphicsDevices, &GraphicsDeviceSelection::choicesChanged);
-#if defined(Q_OS_LINUX) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
     NativeStreamRuntime::initializeDiagnostics();
     LinuxVulkanGraphics::Device vulkanDevice;
 #ifdef NO_SOFT_RENDERING
@@ -237,7 +237,7 @@ static int runApplicationSession(int argc, char *argv[
 #endif
 #ifdef OPENNOW_EMBEDDED_STREAMER
     NativeStreamRuntime nativeStreamRuntime(nullptr,
-#if defined(Q_OS_LINUX) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
                                            vulkanDevice.handle(),
 #else
                                            nullptr,
@@ -402,7 +402,7 @@ static int runApplicationSession(int argc, char *argv[
 #ifdef OPENNOW_EMBEDDED_STREAMER
     streamPresentation.attach(rootWindow);
 #endif
-#if defined(Q_OS_LINUX) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
+#if (defined(Q_OS_LINUX) || defined(Q_OS_FREEBSD)) && QT_CONFIG(vulkan) && __has_include(<vulkan/vulkan.h>)
 #ifdef NO_SOFT_RENDERING
     if (!rootWindow || rootWindow->rendererInterface()->graphicsApi() != QSGRendererInterface::Vulkan) {
         qCritical("NO_SOFT_RENDERING requires the Qt Vulkan scene graph; software and other graphics APIs are forbidden.");
