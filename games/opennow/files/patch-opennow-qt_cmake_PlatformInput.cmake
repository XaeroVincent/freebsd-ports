--- opennow-qt/cmake/PlatformInput.cmake.orig
+++ opennow-qt/cmake/PlatformInput.cmake
@@ -21,7 +21,7 @@
     target_link_libraries(opennow-platform-input PRIVATE "-framework AppKit" "-framework CoreGraphics")
 endif()
 
-if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
+if(CMAKE_SYSTEM_NAME MATCHES "^(Linux|FreeBSD)$")
     enable_language(C)
     find_package(PkgConfig REQUIRED)
     pkg_check_modules(WAYLAND_CLIENT REQUIRED IMPORTED_TARGET wayland-client)
