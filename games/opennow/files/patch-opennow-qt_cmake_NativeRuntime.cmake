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
@@ -132,14 +132,14 @@
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
     list(APPEND OPENNOW_STREAMER_CARGO_FEATURE_ARGS
-        --features linux-ffmpeg-bundled,linux-vaapi)
+        --features linux-ffmpeg,linux-vaapi)
     if(NO_SOFT_RENDERING)
         list(APPEND OPENNOW_STREAMER_CARGO_FEATURE_ARGS --features hardware-only)
     endif()
