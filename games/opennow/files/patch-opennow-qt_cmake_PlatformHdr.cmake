--- opennow-qt/cmake/PlatformHdr.cmake.orig
+++ opennow-qt/cmake/PlatformHdr.cmake
@@ -21,7 +21,7 @@
         "-framework AppKit" "-framework QuartzCore" "-framework CoreGraphics")
 endif()
 
-if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
+if(CMAKE_SYSTEM_NAME MATCHES "^(Linux|FreeBSD)$")
     find_package(PkgConfig QUIET)
     if(PkgConfig_FOUND)
         pkg_check_modules(HDR_WAYLAND_CLIENT QUIET IMPORTED_TARGET wayland-client)
