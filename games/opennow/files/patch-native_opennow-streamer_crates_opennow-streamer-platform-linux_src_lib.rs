--- native/opennow-streamer/crates/opennow-streamer-platform-linux/src/lib.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform-linux/src/lib.rs
@@ -1,71 +1,71 @@
 #[cfg(all(
-    target_os = "linux",
+    any(target_os = "linux", target_os = "freebsd"),
     not(any(target_arch = "x86_64", target_arch = "aarch64"))
 ))]
 compile_error!("the Linux backend currently supports x86_64 and aarch64");
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod audio;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod audio_devices;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod capability;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod error;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod format;
-#[cfg(all(target_os = "linux", feature = "vulkan"))]
+#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "vulkan"))]
 mod frame_producer;
-#[cfg(all(target_os = "linux", feature = "vulkan"))]
+#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "vulkan"))]
 mod presentation;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod queue;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod session;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod timing;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod video;
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod vulkan_device;
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use video::{
     MEDIACODEC_MAX_HEIGHT, MEDIACODEC_MAX_WIDTH, VaapiColorSupport, probe_mediacodec,
     vaapi_color_support,
 };
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use vulkan_device::{SharedVulkanDevice, VulkanDeviceInfo};
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use audio::{AudioBackend, AudioBackendPreference, AudioConfig, AudioPacket};
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use audio_devices::{AudioOutputDevice, audio_output_devices};
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use capability::{
     BackendCapability, CapabilityReport, PresentationCapability, probe_capabilities,
     probe_video_capabilities,
 };
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use error::{Error, Result, Subsystem};
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use format::{
     ChromaLocation, ColorMatrix, ColorPrimaries, ColorRange, ColorTransfer, DecodedVideoFrame,
     DmaBufFrame, DmaBufLayer, DmaBufObject, DmaBufPlane, EncodedVideoFrame, FramePlane,
     PixelFormat, StreamFormat, VideoCodec, VulkanImage, VulkanVideoFrame,
 };
-#[cfg(all(target_os = "linux", feature = "vulkan"))]
+#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "vulkan"))]
 pub use frame_producer::{
     CpuNv12Frame, GpuTextureFormat, ImportedNv12Frame, LinuxFrameProducer, LinuxGpuFrame,
     LinuxGpuFrameProducer, LinuxGpuRenderResources, LinuxTextureColorSpace, PreparedLinuxFrame,
     PreparedVulkanFrame, PreparedVulkanImage, RecordedGpuFrame, VulkanRenderDevice,
 };
-#[cfg(all(target_os = "linux", feature = "vulkan"))]
+#[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "vulkan"))]
 pub use presentation::{NativeSurface, VulkanPresenter};
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use session::{
     BackendEvent, DecoderBackend, DecoderPreference, LifecycleState, LinuxSession, PushOutcome,
     SessionConfig,
 };
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub use timing::{DecodeStagePercentiles, DecodeTimingProbe, DecodeTimings};
