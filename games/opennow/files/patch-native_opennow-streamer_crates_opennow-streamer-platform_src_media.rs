--- native/opennow-streamer/crates/opennow-streamer-platform/src/media.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/media.rs
@@ -21,7 +21,7 @@
 use crate::runtime::HostCommand;
 use crate::video_queue::VideoQueue;
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use crate::linux_backend::{LinuxVideoPath, LinuxVideoSelection};
 
 const VIDEO_QUEUE_CAPACITY: usize = 2;
@@ -486,7 +486,7 @@
         matches!(self, Self::EightBit444 | Self::TenBit444)
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     pub(crate) const fn linux_pixel_format(self) -> opennow_streamer_platform_linux::PixelFormat {
         use opennow_streamer_platform_linux::PixelFormat;
         match self {
@@ -497,7 +497,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     pub(crate) const fn from_linux_pixel_format(
         format: opennow_streamer_platform_linux::PixelFormat,
     ) -> Option<Self> {
@@ -945,13 +945,13 @@
     mac_software_fallback: AtomicBool,
     #[cfg(target_os = "windows")]
     windows_bridge: Arc<WindowsBridge>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_session: Mutex<Option<opennow_streamer_platform_linux::LinuxSession>>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_software_fallback: Arc<AtomicBool>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_video_mid: Mutex<String>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_codec: MediaVideoCodec,
 }
 
@@ -1033,7 +1033,7 @@
     sink: MediaSink,
     video_worker: Option<JoinHandle<()>>,
     audio_worker: Option<JoinHandle<()>>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_monitor: Option<JoinHandle<()>>,
     embedded_host_worker: Option<JoinHandle<()>>,
     #[cfg(target_os = "windows")]
@@ -1060,10 +1060,10 @@
         use_hardware: bool,
         stream: MediaStreamConfig,
         #[cfg(target_os = "windows")] windows_bridge: Arc<WindowsBridge>,
-        #[cfg(target_os = "linux")] linux_selection: LinuxVideoSelection,
-        #[cfg(target_os = "linux")] linux_software_fallback: Arc<AtomicBool>,
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))] linux_selection: LinuxVideoSelection,
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))] linux_software_fallback: Arc<AtomicBool>,
     ) -> Result<Self, String> {
-        #[cfg(not(target_os = "linux"))]
+        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
         let _ = &stream;
         #[cfg(target_os = "macos")]
         if use_hardware {
@@ -1077,7 +1077,7 @@
         let use_windows_backend = false;
         #[cfg(not(any(target_os = "windows", target_os = "macos")))]
         let _ = use_hardware;
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if !linux_software_fallback.load(Ordering::Acquire)
             && let LinuxVideoPath::Hardware(decoder_preference) = linux_selection.path
         {
@@ -1133,13 +1133,13 @@
             mac_software_fallback: AtomicBool::new(false),
             #[cfg(target_os = "windows")]
             windows_bridge,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_session: Mutex::new(None),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_software_fallback,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_video_mid: Mutex::new(String::new()),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_codec: stream.codec,
         });
         let video_shared = Arc::clone(&shared);
@@ -1173,7 +1173,7 @@
             sink: MediaSink { shared },
             video_worker: Some(video_worker),
             audio_worker: Some(audio_worker),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_monitor: None,
             embedded_host_worker: None,
             #[cfg(target_os = "windows")]
@@ -1242,7 +1242,7 @@
             sink: MediaSink { shared },
             video_worker: Some(video_worker),
             audio_worker: Some(audio_worker),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_monitor: None,
             embedded_host_worker: None,
             embedded_frames: None,
@@ -1250,7 +1250,7 @@
         })
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn spawn_linux(
         output: Arc<OutputBuffers>,
         feedback: Sender<MediaFeedback>,
@@ -1344,10 +1344,10 @@
         host_commands: Sender<HostCommand>,
         config: MediaSessionConfig<'_>,
         frames: crate::GraphicsFramePublisher,
-        #[cfg(target_os = "linux")] linux_selection: LinuxVideoSelection,
-        #[cfg(target_os = "linux")] vulkan_device: Option<Arc<crate::SharedVulkanDevice>>,
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))] linux_selection: LinuxVideoSelection,
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))] vulkan_device: Option<Arc<crate::SharedVulkanDevice>>,
     ) -> Result<Self, String> {
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             return Self::spawn_embedded_linux(
                 output,
@@ -1384,7 +1384,7 @@
         Err("embedded media is unsupported on this platform".to_owned())
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn spawn_embedded_linux(
         output: Arc<OutputBuffers>,
         feedback: Sender<MediaFeedback>,
@@ -1960,7 +1960,7 @@
         host_commands: Sender<HostCommand>,
         stream: MediaStreamConfig,
         #[cfg(target_os = "windows")] windows_bridge: Arc<WindowsBridge>,
-        #[cfg(target_os = "linux")] linux_software_fallback: Arc<AtomicBool>,
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))] linux_software_fallback: Arc<AtomicBool>,
     ) -> Result<Self, String> {
         let shared = Arc::new(SharedPipeline {
             video: Arc::new(VideoQueue::new(video_queue_capacity(stream.fps))),
@@ -1980,13 +1980,13 @@
             mac_software_fallback: AtomicBool::new(false),
             #[cfg(target_os = "windows")]
             windows_bridge,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_session: Mutex::new(None),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_software_fallback,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_video_mid: Mutex::new(String::new()),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_codec: stream.codec,
         });
         let video_shared = Arc::clone(&shared);
@@ -2014,7 +2014,7 @@
             sink: MediaSink { shared },
             video_worker: Some(video_worker),
             audio_worker: Some(audio_worker),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_monitor: None,
             embedded_host_worker: None,
             #[cfg(target_os = "windows")]
@@ -2050,7 +2050,7 @@
                 .unwrap_or_else(|error| error.into_inner())
                 .reset();
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Some(session) = self
             .sink
             .shared
@@ -2089,7 +2089,7 @@
         if let Some(worker) = self.audio_worker.take() {
             let _ = worker.join();
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Some(worker) = self.linux_monitor.take() {
             let _ = worker.join();
         }
@@ -3299,7 +3299,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn run_linux_video(shared: Arc<SharedPipeline>, host_commands: Sender<HostCommand>) {
     while let Some(frame) = shared.video.pop() {
         if shared.linux_software_fallback.load(Ordering::Acquire) {
@@ -3377,7 +3377,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn run_embedded_linux_video(shared: Arc<SharedPipeline>) {
     while let Some(frame) = shared.video.pop() {
         if shared.paused.load(Ordering::Acquire) {
@@ -3440,7 +3440,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn run_embedded_linux_audio(shared: Arc<SharedPipeline>) {
     while let Some(frame) = shared.audio.pop() {
         if shared.paused.load(Ordering::Acquire) {
@@ -3511,7 +3511,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn run_embedded_linux_monitor(
     shared: Arc<SharedPipeline>,
     publisher: crate::GraphicsFramePublisher,
@@ -3697,7 +3697,7 @@
     stop_linux_session(&shared);
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn run_linux_monitor(shared: Arc<SharedPipeline>, host_commands: Sender<HostCommand>) {
     use std::time::Duration;
 
@@ -3820,7 +3820,7 @@
     stop_linux_session(&shared);
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn trigger_linux_fallback(
     shared: &SharedPipeline,
     host_commands: &Sender<HostCommand>,
@@ -3843,7 +3843,7 @@
     let _ = host_commands.send(HostCommand::FallbackLinux { reason });
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn request_linux_keyframe(shared: &SharedPipeline, reason: &str) {
     shared.video_desynced.store(true, Ordering::Release);
     if shared.keyframe_requested.swap(true, Ordering::AcqRel) {
@@ -3862,7 +3862,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn stop_linux_session(shared: &SharedPipeline) {
     if let Some(mut session) = shared
         .linux_session
@@ -3874,7 +3874,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn report_linux_color_format_change(
     shared: &SharedPipeline,
     reported: &mut Option<MediaColorQuality>,
@@ -3893,7 +3893,7 @@
     });
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 const fn linux_decoder_name(
     backend: opennow_streamer_platform_linux::DecoderBackend,
 ) -> &'static str {
@@ -3909,7 +3909,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 const fn linux_embedded_backend_label(
     preference: opennow_streamer_platform_linux::DecoderPreference,
 ) -> &'static str {
@@ -3924,7 +3924,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 const fn linux_audio_backend_name(
     backend: opennow_streamer_platform_linux::AudioBackend,
 ) -> &'static str {
@@ -3934,7 +3934,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn forward_linux_audio_output_loss(
     shared: &SharedPipeline,
     backend: opennow_streamer_platform_linux::AudioBackend,
@@ -3947,7 +3947,7 @@
     });
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn forward_linux_audio_output_recovery(
     shared: &SharedPipeline,
     from: opennow_streamer_platform_linux::AudioBackend,
@@ -3964,7 +3964,7 @@
     });
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 const fn linux_subsystem_name(
     subsystem: opennow_streamer_platform_linux::Subsystem,
 ) -> &'static str {
@@ -3981,7 +3981,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn media_timestamp_us(timestamp: u64, clock_rate_hz: u32) -> u64 {
     if clock_rate_hz == 0 {
         return 0;
@@ -4724,7 +4724,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn linux_stream_color_profiles_preserve_chroma_and_bit_depth() {
         use opennow_streamer_platform_linux::PixelFormat;
@@ -4742,7 +4742,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn linux_decoded_formats_without_a_color_class_are_not_guessed() {
         use opennow_streamer_platform_linux::PixelFormat;
@@ -4755,7 +4755,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn linux_format_change_reports_the_decoded_color_class_once_per_change() {
         use opennow_streamer_platform_linux::{ColorTransfer, PixelFormat, StreamFormat};
@@ -4823,7 +4823,7 @@
         assert!(receiver.try_recv().is_err());
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn embedded_linux_rejects_color_downgrades_and_missing_vulkan_owner() {
         use opennow_streamer_platform_linux::DecoderPreference;
@@ -4895,7 +4895,7 @@
         }
     }
 
-    #[cfg(all(target_os = "linux", feature = "hardware-only"))]
+    #[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "hardware-only"))]
     #[test]
     fn hardware_only_embedded_startup_rejects_no_decoder_and_software_before_workers() {
         use opennow_streamer_platform_linux::DecoderPreference;
@@ -4935,7 +4935,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn device_lost_feedback(
         receiver: &Receiver<MediaFeedback>,
     ) -> (&'static str, bool, Option<String>) {
@@ -4952,7 +4952,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn linux_audio_output_feedback_pairs_loss_and_recovery_without_touching_video() {
         use opennow_streamer_platform_linux::AudioBackend;
@@ -4990,7 +4990,7 @@
         assert!(shared.video_desynced.load(Ordering::Acquire));
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn linux_audio_output_recovery_pairs_a_backend_that_recovered_itself() {
         use opennow_streamer_platform_linux::AudioBackend;
@@ -5566,19 +5566,19 @@
             mac_software_fallback: AtomicBool::new(true),
             #[cfg(target_os = "windows")]
             windows_bridge: Arc::new(WindowsBridge::new()),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_session: Mutex::new(None),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_software_fallback: Arc::new(AtomicBool::new(true)),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_video_mid: Mutex::new(String::new()),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_codec: MediaVideoCodec::H264,
         });
         (shared, receiver)
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn hardware_only_standalone_recovery_never_starts_openh264_or_sdl() {
         let (shared, feedback) = software_test_pipeline();
@@ -5900,7 +5900,7 @@
             sink: MediaSink { shared },
             video_worker: None,
             audio_worker: Some(worker),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_monitor: None,
             embedded_host_worker: None,
             #[cfg(target_os = "windows")]
@@ -5966,7 +5966,7 @@
         replacement.stop();
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn fatal_standalone_video_still_closes_audio_and_completes_stop() {
         use std::time::Duration;
@@ -6075,13 +6075,13 @@
             MediaStreamConfig::default(),
             #[cfg(target_os = "windows")]
             Arc::new(WindowsBridge::new()),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             LinuxVideoSelection {
                 path: LinuxVideoPath::Software,
                 use_vulkan_output: false,
                 fallback_reason: None,
             },
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Arc::new(AtomicBool::new(true)),
         );
         if crate::HARDWARE_ONLY {
@@ -6134,13 +6134,13 @@
             MediaStreamConfig::default(),
             #[cfg(target_os = "windows")]
             Arc::new(WindowsBridge::new()),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             LinuxVideoSelection {
                 path: LinuxVideoPath::Software,
                 use_vulkan_output: false,
                 fallback_reason: None,
             },
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Arc::new(AtomicBool::new(true)),
         );
         if crate::HARDWARE_ONLY {
@@ -6169,7 +6169,7 @@
         session.stop();
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn converts_rtp_timestamps_to_microseconds_without_overflow() {
         assert_eq!(media_timestamp_us(180_000, 90_000), 2_000_000);
