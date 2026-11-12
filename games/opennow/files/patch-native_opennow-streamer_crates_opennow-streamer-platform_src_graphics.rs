--- native/opennow-streamer/crates/opennow-streamer-platform/src/graphics.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/graphics.rs
@@ -150,14 +150,14 @@
     ) -> Result<GraphicsRecordedFrame, GraphicsFrameError>;
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl GraphicsRenderResources for opennow_streamer_platform_linux::LinuxGpuRenderResources {
     fn retire(&self) -> Result<(), String> {
         self.retire().map_err(|error| error.to_string())
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl GraphicsFrame for opennow_streamer_platform_linux::LinuxGpuFrame {
     fn render_resources(&self) -> Option<Arc<dyn GraphicsRenderResources>> {
         Some(self.render_resources())
