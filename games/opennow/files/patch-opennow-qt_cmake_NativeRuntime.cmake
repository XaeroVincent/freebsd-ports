--- opennow-qt/cmake/NativeRuntime.cmake.orig
+++ opennow-qt/cmake/NativeRuntime.cmake
@@ -84,7 +84,7 @@
 
 option(NO_SOFT_RENDERING "Forbid software video decoding and graphics composition on Linux" OFF)
 if(NO_SOFT_RENDERING)
-    if(NOT CMAKE_SYSTEM_NAME STREQUAL "Linux")
+    if(NOT CMAKE_SYSTEM_NAME MATCHES "^(Linux|FreeBSD)$")
         message(FATAL_ERROR "NO_SOFT_RENDERING is supported only on Linux")
     endif()
     add_compile_definitions(NO_SOFT_RENDERING)
@@ -132,21 +132,29 @@
 # into every Qt configuration.
 set(OPENNOW_STREAMER_PROFILE "release")
 set(OPENNOW_STREAMER_CARGO_FEATURE_ARGS)
-if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
+if(CMAKE_SYSTEM_NAME MATCHES "^(Linux|FreeBSD)$")
     # Distributable Linux builds carry FFmpeg's H.264/HEVC/AV1 software
     # fallback and native H.264 VAAPI with host-provided GPU drivers. Every Qt
     # configuration now shares one cargo profile directory, so this feature has
     # to stay unconditional: gating it on Release would make each Debug/Release
     # switch rebuild the vendored FFmpeg instead of paying for it once.
-    list(APPEND OPENNOW_STREAMER_CARGO_FEATURE_ARGS
-        --features linux-ffmpeg-bundled,linux-vaapi)
+    if(CMAKE_SYSTEM_NAME STREQUAL "FreeBSD")
+        # FreeBSD packages link the system FFmpeg instead of building the vendored copy.
+        list(APPEND OPENNOW_STREAMER_CARGO_FEATURE_ARGS
+            --features linux-ffmpeg,linux-vaapi)
+    else()
+        list(APPEND OPENNOW_STREAMER_CARGO_FEATURE_ARGS
+            --features linux-ffmpeg-bundled,linux-vaapi)
+    endif()
     if(NO_SOFT_RENDERING)
         list(APPEND OPENNOW_STREAMER_CARGO_FEATURE_ARGS --features hardware-only)
     endif()
     find_package(PkgConfig REQUIRED)
     pkg_check_modules(OPENNOW_VAAPI REQUIRED libva libva-drm libdrm)
-    if(OPENNOW_RUST_TARGET MATCHES "^aarch64-.*-linux-"
-        OR (NOT OPENNOW_RUST_TARGET AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(aarch64|arm64|ARM64)$"))
+    # The V4L2 request decoder is Linux-only.
+    if(CMAKE_SYSTEM_NAME STREQUAL "Linux"
+        AND (OPENNOW_RUST_TARGET MATCHES "^aarch64-.*-linux-"
+            OR (NOT OPENNOW_RUST_TARGET AND CMAKE_SYSTEM_PROCESSOR MATCHES "^(aarch64|arm64|ARM64)$")))
         pkg_check_modules(OPENNOW_V4L2_REQUEST REQUIRED libdrm libudev)
     endif()
 endif()
