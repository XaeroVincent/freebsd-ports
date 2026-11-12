--- native/opennow-streamer/crates/opennow-streamer-platform/src/lib.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/lib.rs
@@ -2,11 +2,11 @@
 mod embedded_input;
 mod graphics;
 mod h264;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod linux_backend;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod linux_frame_pacing;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod linux_xinput;
 #[cfg(target_os = "macos")]
 mod macos_backend;
@@ -41,11 +41,11 @@
     EncodedMicrophoneFrame, MICROPHONE_FRAME_SAMPLES, MICROPHONE_SAMPLE_RATE,
     MicrophoneDiagnostics, MicrophoneReceiver, MicrophoneSession, MicrophoneStatus,
 };
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use opennow_streamer_platform_linux::{
     LinuxGpuFrame, LinuxGpuFrameProducer, SharedVulkanDevice,
 };
-#[cfg(not(target_os = "linux"))]
+#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
 pub enum SharedVulkanDevice {}
 #[cfg(target_os = "macos")]
 pub use opennow_streamer_platform_macos::{
@@ -74,7 +74,7 @@
     CodecCapability, GraphicsAdapterCapability, VideoBackendCapability,
 };
 
-const HARDWARE_ONLY: bool = cfg!(all(target_os = "linux", feature = "hardware-only"));
+const HARDWARE_ONLY: bool = cfg!(all(any(target_os = "linux", target_os = "freebsd"), feature = "hardware-only"));
 const HARDWARE_ONLY_ERROR: &str =
     "hardware-only build forbids software video decoding and composition";
 
@@ -93,7 +93,7 @@
 }
 
 pub fn video_backends() -> Vec<VideoBackendCapability> {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let mut backends = {
         let mut backends = linux_backend::video_backends();
         backends.push(software_backend());
@@ -120,7 +120,7 @@
             software_backend(),
         ]
     };
-    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
+    #[cfg(not(any(any(target_os = "linux", target_os = "freebsd"), target_os = "windows")))]
     let mut backends = vec![hardware_backend(), software_backend()];
     apply_backend_policy(
         &mut backends,
@@ -139,11 +139,11 @@
     _device: Option<&SharedVulkanDevice>,
     _windows_adapter_luid: Option<WindowsAdapterLuid>,
 ) -> Vec<VideoBackendCapability> {
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let mut backends = linux_backend::mediacodec_capability()
         .map(|backend| vec![backend])
         .unwrap_or_else(linux_backend::video_backends);
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     for backend in &mut backends {
         if backend.backend == "vulkan" {
             use opennow_streamer_platform_linux::{PixelFormat, VideoCodec};
@@ -217,7 +217,7 @@
             }
         }
     }
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     for backend in &mut backends {
         for codec in &mut backend.codecs {
             codec.hdr_color_qualities = Some(
@@ -296,7 +296,7 @@
             codec.color_qualities = Some(colors);
         }
     }
-    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
+    #[cfg(not(any(any(target_os = "linux", target_os = "freebsd"), target_os = "macos", target_os = "windows")))]
     let mut backends = Vec::new();
     apply_backend_policy(
         &mut backends,
@@ -307,7 +307,7 @@
     backends
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn apply_vaapi_color_support(
     colors: &mut Vec<&'static str>,
     support: opennow_streamer_platform_linux::VaapiColorSupport,
@@ -548,7 +548,7 @@
     }
 }
 
-#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
+#[cfg(not(any(target_os = "windows", target_os = "macos", any(target_os = "linux", target_os = "freebsd"))))]
 fn hardware_backend() -> VideoBackendCapability {
     unavailable_backend("unsupported", "other", "Unsupported operating system")
 }
@@ -644,7 +644,7 @@
     }
 }
 
-#[cfg(not(target_os = "linux"))]
+#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
 fn unavailable_backend(
     backend: &'static str,
     platform: &'static str,
@@ -888,7 +888,7 @@
                 .filter(|codec| codec.codec != "h264")
                 .all(|codec| !codec.available)
         );
-        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
+        #[cfg(not(any(target_os = "macos", target_os = "windows", any(target_os = "linux", target_os = "freebsd"))))]
         assert!(
             backends
                 .iter()
@@ -990,7 +990,7 @@
     fn embedded_capabilities_exclude_standalone_only_backends() {
         let backends = embedded_video_backends();
         assert!(backends.iter().all(|backend| backend.backend != "software"));
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             let vulkan = backends
                 .iter()
@@ -1018,7 +1018,7 @@
         );
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn embedded_linux_hdr_requires_an_available_ten_bit_output_path() {
         for backend in embedded_video_backends_with_config(None, None) {
@@ -1072,7 +1072,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn embedded_linux_reports_explicit_color_profiles_without_claiming_zero_copy() {
         for backend in embedded_video_backends_with_config(None, None) {
@@ -1111,7 +1111,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn vaapi_color_support_extends_the_advertised_profiles_in_wire_order() {
         use opennow_streamer_platform_linux::VaapiColorSupport;
