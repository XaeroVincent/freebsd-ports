--- native/opennow-streamer/crates/opennow-streamer-ffi/src/lib.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-ffi/src/lib.rs
@@ -243,7 +243,7 @@
             return OpenNowStreamerStatus::NullPointer;
         }
         unsafe { output.write(ptr::null_mut()) };
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             let device = match SharedVulkanDevice::create() {
                 Ok(device) => device,
@@ -276,7 +276,7 @@
             };
             OpenNowStreamerStatus::Ok
         }
-        #[cfg(not(target_os = "linux"))]
+        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
         OpenNowStreamerStatus::GraphicsUnavailable
     })) {
         Ok(status) => status,
@@ -298,7 +298,7 @@
         if handle.is_null() || output.is_null() {
             return OpenNowStreamerStatus::NullPointer;
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             let info = unsafe { &*handle }.device.info();
             unsafe {
@@ -316,7 +316,7 @@
             }
             OpenNowStreamerStatus::Ok
         }
-        #[cfg(not(target_os = "linux"))]
+        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
         OpenNowStreamerStatus::GraphicsUnavailable
     })) {
         Ok(status) => status,
@@ -1821,7 +1821,7 @@
         };
         let precision = if cfg!(target_os = "windows") {
             Some(PresentationTimestampPrecision::Windows)
-        } else if cfg!(target_os = "linux") {
+        } else if cfg!(any(target_os = "linux", target_os = "freebsd")) {
             Some(PresentationTimestampPrecision::Linux)
         } else if cfg!(target_os = "macos") {
             Some(PresentationTimestampPrecision::MacOs)
@@ -2414,7 +2414,7 @@
     }
 
     #[cfg(not(all(
-        target_os = "linux",
+        any(target_os = "linux", target_os = "freebsd"),
         any(feature = "linux-ffmpeg", feature = "linux-ffmpeg-bundled")
     )))]
     #[test]
