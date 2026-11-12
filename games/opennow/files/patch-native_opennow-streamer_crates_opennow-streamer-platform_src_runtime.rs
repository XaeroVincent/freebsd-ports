--- native/opennow-streamer/crates/opennow-streamer-platform/src/runtime.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/runtime.rs
@@ -13,7 +13,7 @@
 use crate::output::WindowsBridge;
 use crate::output::{ActiveOutput, OutputBuffers, OutputControl, OutputEvent};
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use crate::linux_backend::{LinuxVideoPath, LinuxVideoSelection};
 
 // Keep native input sampling below one rendered frame even at 120 Hz. SDL can
@@ -87,7 +87,7 @@
         format: opennow_streamer_platform_macos::Av1Format,
         reply: Sender<Result<opennow_streamer_platform_macos::StreamSink, String>>,
     },
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     FallbackLinux {
         reason: String,
     },
@@ -110,9 +110,9 @@
     use_hardware: Arc<AtomicBool>,
     #[cfg(target_os = "windows")]
     windows_bridge: Arc<WindowsBridge>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_selection: LinuxVideoSelection,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_software_fallback: Arc<AtomicBool>,
     mode: MediaRuntimeMode,
 }
@@ -236,7 +236,7 @@
         let supported = matches!(requested, "auto" | "d3d11");
         #[cfg(target_os = "macos")]
         let supported = matches!(requested, "auto" | "videotoolbox");
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         let supported = requested == "auto"
             || self.video_backends().iter().any(|backend| {
                 backend.available
@@ -245,7 +245,7 @@
                         || (matches!(requested, "software" | "ffmpeg")
                             && backend.backend == "ffmpeg"))
             });
-        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
+        #[cfg(not(any(target_os = "windows", target_os = "macos", any(target_os = "linux", target_os = "freebsd"))))]
         let supported = requested == "auto";
         if supported {
             Ok(())
@@ -284,7 +284,7 @@
             }
             if !cfg!(any(
                 target_os = "windows",
-                target_os = "linux",
+                any(target_os = "linux", target_os = "freebsd"),
                 target_os = "macos"
             )) {
                 return Err(
@@ -357,13 +357,13 @@
                     audio_device: &audio_device,
                 },
                 frames.clone(),
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 crate::linux_backend::select_embedded_video_path(
                     requested,
                     _config.vulkan_device.as_deref(),
                     stream,
                 ),
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 _config.vulkan_device.clone(),
             )
             .inspect_err(|_| {
@@ -383,7 +383,7 @@
                 stream,
                 #[cfg(target_os = "windows")]
                 Arc::clone(&self.windows_bridge),
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 Arc::clone(&self.linux_software_fallback),
             )?;
             if self.paused.load(Ordering::Acquire) {
@@ -403,7 +403,7 @@
         response
             .recv_timeout(HOST_START_TIMEOUT)
             .map_err(|_| "native media host did not start on the UI thread".to_owned())??;
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Some(reason) = self.linux_selection.fallback_reason.clone() {
             let _ = feedback.send(MediaFeedback::BackendFallback {
                 from: "Linux Vulkan presentation",
@@ -423,9 +423,9 @@
             stream,
             #[cfg(target_os = "windows")]
             Arc::clone(&self.windows_bridge),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             self.linux_selection.clone(),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Arc::clone(&self.linux_software_fallback),
         ) {
             Ok(session) => {
@@ -454,7 +454,7 @@
             )
             .supports_format(VideoCodec::H265, VideoPixelFormat::Y410, true)
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             use opennow_streamer_platform_linux::{PixelFormat, VideoCodec};
             matches!(&self.mode, MediaRuntimeMode::Embedded { config, .. }
@@ -464,7 +464,7 @@
         {
             opennow_streamer_platform_macos::probe_h265_hdr_444_hardware()
         }
-        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
+        #[cfg(not(any(target_os = "windows", any(target_os = "linux", target_os = "freebsd"), target_os = "macos")))]
         false
     }
 
@@ -524,9 +524,9 @@
     use_macos_hardware: Arc<AtomicBool>,
     #[cfg(target_os = "windows")]
     windows_bridge: Arc<WindowsBridge>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_selection: LinuxVideoSelection,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_software_fallback: Arc<AtomicBool>,
 }
 
@@ -572,11 +572,11 @@
                         output.stop();
                     }
                     let requested_hardware = self.use_macos_hardware.load(Ordering::Acquire);
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     let requested_linux_hardware = self.linux_selection.use_vulkan_output
                         && matches!(self.linux_selection.path, LinuxVideoPath::Hardware(_))
                         && !self.linux_software_fallback.load(Ordering::Acquire);
-                    #[cfg(not(target_os = "linux"))]
+                    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
                     let requested_linux_hardware = false;
                     let mut startup_fallback = None;
                     let initialized = match initialize_output(
@@ -720,7 +720,7 @@
                             new_surface.visible, new_surface.screen_rect
                         );
                     }
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     let linux_hardware =
                         active.as_ref().is_some_and(ActiveOutput::is_linux_hardware);
                     let result = if let Some(output) = active.as_mut() {
@@ -728,9 +728,9 @@
                     } else {
                         Ok(())
                     };
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     let mut result = result;
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     if linux_hardware && let Err(reason) = &result {
                         let replacement =
                             retry_software_output(crate::HARDWARE_ONLY, reason.clone(), || {
@@ -873,7 +873,7 @@
                         output.update_cursor(&update.bytes);
                     }
                 }
-                #[cfg(target_os = "linux")]
+                #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                 Ok(HostCommand::FallbackLinux { reason }) => {
                     let replacement =
                         retry_software_output(crate::HARDWARE_ONLY, reason.clone(), || {
@@ -1114,7 +1114,7 @@
                                 }
                             }
                         }
-                        #[cfg(target_os = "linux")]
+                        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                         if output.is_linux_hardware() {
                             let fallback = retry_software_output(
                                 crate::HARDWARE_ONLY,
@@ -1170,9 +1170,9 @@
 pub fn create_runtime() -> Result<(MainThreadHost, MediaRuntime), String> {
     ensure_macos_main_thread()?;
     let use_macos_hardware = Arc::new(AtomicBool::new(use_hardware_backend()));
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let linux_selection = crate::linux_backend::select_video_path();
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     if crate::HARDWARE_ONLY
         && (matches!(linux_selection.path, LinuxVideoPath::Software)
             || !linux_selection.use_vulkan_output)
@@ -1186,7 +1186,7 @@
                 .unwrap_or("hardware Vulkan presentation is unavailable")
         ));
     }
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let linux_software_fallback = Arc::new(AtomicBool::new(matches!(
         linux_selection.path,
         LinuxVideoPath::Software
@@ -1204,9 +1204,9 @@
             use_macos_hardware: Arc::clone(&use_macos_hardware),
             #[cfg(target_os = "windows")]
             windows_bridge: Arc::clone(&windows_bridge),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_selection: linux_selection.clone(),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_software_fallback: Arc::clone(&linux_software_fallback),
         },
         MediaRuntime {
@@ -1217,9 +1217,9 @@
             use_hardware: use_macos_hardware,
             #[cfg(target_os = "windows")]
             windows_bridge,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_selection,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_software_fallback,
             mode: MediaRuntimeMode::Standalone,
         },
@@ -1296,7 +1296,7 @@
                         }
                         HostCommand::AudioDevices(request) => {
                             let reply = &request.reply;
-                            #[cfg(all(target_os = "linux", not(test)))]
+                            #[cfg(all(any(target_os = "linux", target_os = "freebsd"), not(test)))]
                             let devices = opennow_streamer_platform_linux::audio_output_devices()
                                 .map(|devices| {
                                     devices
@@ -1323,7 +1323,7 @@
                                 })
                                 .map_err(|error| error.to_string());
                             #[cfg(not(any(
-                                target_os = "linux",
+                                any(target_os = "linux", target_os = "freebsd"),
                                 target_os = "windows",
                                 target_os = "macos",
                                 test
@@ -1383,7 +1383,7 @@
             host_output.stop_microphone();
             drop(microphone);
         });
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let linux_selection = crate::linux_backend::select_video_path();
     MediaRuntime {
         commands,
@@ -1393,9 +1393,9 @@
         use_hardware: Arc::new(AtomicBool::new(true)),
         #[cfg(target_os = "windows")]
         windows_bridge: Arc::new(WindowsBridge::new()),
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         linux_selection,
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         linux_software_fallback: Arc::new(AtomicBool::new(false)),
         mode: MediaRuntimeMode::Embedded { frames, config },
     }
@@ -1416,9 +1416,9 @@
 #[cfg(feature = "test-runtime")]
 pub fn create_test_runtime() -> (TestMediaRuntimeHost, MediaRuntime) {
     let use_macos_hardware = Arc::new(AtomicBool::new(false));
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let linux_selection = crate::linux_backend::select_video_path();
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     let linux_software_fallback = Arc::new(AtomicBool::new(matches!(
         linux_selection.path,
         LinuxVideoPath::Software
@@ -1469,7 +1469,7 @@
                     HostCommand::ConfigureMacAv1 { reply, .. } => {
                         let _ = reply.send(Err("test media runtime has no decoder".to_owned()));
                     }
-                    #[cfg(target_os = "linux")]
+                    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
                     HostCommand::FallbackLinux { .. } => {}
                     HostCommand::Shutdown => break,
                 }
@@ -1488,9 +1488,9 @@
             use_hardware: use_macos_hardware,
             #[cfg(target_os = "windows")]
             windows_bridge,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_selection,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_software_fallback,
             mode: MediaRuntimeMode::Test,
         },
@@ -1647,7 +1647,7 @@
         assert_eq!(attempts.get(), 2);
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn hardware_only_runtime_rejects_explicit_software_settings_before_startup() {
         let (_graphics, frames) = crate::RenderThreadGraphics::new(|| {});
@@ -1981,7 +1981,7 @@
         runtime.shutdown();
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn hdr_444_without_shared_vulkan_device_never_falls_back_to_cpu() {
         let (_graphics, frames) = crate::RenderThreadGraphics::new(|| {});
@@ -2153,7 +2153,7 @@
         host.join().unwrap();
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn embedded_runtime_without_an_owner_disables_vulkan() {
         let (_graphics, frames) = crate::RenderThreadGraphics::new(|| {});
@@ -2170,7 +2170,7 @@
         runtime.shutdown();
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn embedded_runtime_accepts_software_only_when_the_cpu_decoder_is_available() {
         let (_graphics, frames) = crate::RenderThreadGraphics::new(|| {});
