--- native/opennow-streamer/crates/opennow-streamer-platform/src/output.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/output.rs
@@ -1,4 +1,4 @@
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use std::collections::VecDeque;
 use std::collections::{HashMap, HashSet};
 use std::io::Cursor as IoCursor;
@@ -19,16 +19,16 @@
 use sdl2::rect::Rect;
 use sdl2::render::{Texture, WindowCanvas};
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use crate::linux_frame_pacing::{FrameSelectionPolicy, LinuxFramePacer};
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use crate::linux_xinput::LinuxXInputController;
 use crate::media::{CapturedInput, CapturedInputQueue, MediaStreamConfig, StreamShortcutBindings};
 use crate::native_surface::NativeSurface;
 #[cfg(target_os = "windows")]
 use crate::windows_raw_input::WindowsRawInputController;
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 use opennow_streamer_platform_linux::DecodedVideoFrame as LinuxDecodedVideoFrame;
 #[cfg(target_os = "windows")]
 use opennow_streamer_platform_windows::{
@@ -122,7 +122,7 @@
         })
         .map_err(|error| format!("native audio output creation failed: {error}"))
 }
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 const LINUX_VIDEO_QUEUE_CAPACITY: usize = 6;
 
 #[derive(Debug)]
@@ -132,7 +132,7 @@
     pub(crate) rgb: Vec<u8>,
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 #[derive(Debug)]
 struct QueuedLinuxVideoFrame {
     frame: LinuxDecodedVideoFrame,
@@ -142,7 +142,7 @@
 #[derive(Debug)]
 pub(crate) struct OutputBuffers {
     video: Mutex<Option<DecodedVideoFrame>>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     linux_video: Mutex<VecDeque<QueuedLinuxVideoFrame>>,
     audio: Mutex<AudioPlayoutBuffer>,
     #[cfg(test)]
@@ -246,7 +246,7 @@
     pub(crate) fn with_captured_input(captured_input: Arc<CapturedInputQueue>) -> Self {
         Self {
             video: Mutex::new(None),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_video: Mutex::new(VecDeque::with_capacity(LINUX_VIDEO_QUEUE_CAPACITY)),
             audio: Mutex::new(AudioPlayoutBuffer::new(
                 AUDIO_SAMPLE_RATE as usize * AUDIO_CHANNELS as usize * MAX_AUDIO_LATENCY_MS / 1_000,
@@ -316,7 +316,7 @@
             .take()
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     pub(crate) fn queue_linux_video(&self, frame: LinuxDecodedVideoFrame) -> bool {
         let mut queue = self
             .linux_video
@@ -335,7 +335,7 @@
         dropped
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn take_linux_video(&self) -> Option<QueuedLinuxVideoFrame> {
         self.linux_video
             .lock()
@@ -343,7 +343,7 @@
             .pop_front()
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn take_linux_video_for_presentation(
         &self,
         selection: FrameSelectionPolicy,
@@ -366,7 +366,7 @@
         }
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn clear_linux_video(&self) {
         self.linux_video
             .lock()
@@ -395,7 +395,7 @@
 
     pub(crate) fn clear(&self) {
         self.take_video();
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         self.clear_linux_video();
         self.audio
             .lock()
@@ -458,7 +458,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 pub(crate) struct LinuxHardwareOutput {
     // These fields own resources derived from the SDL Wayland/X11 display.
     // Rust drops fields in declaration order, so keep the presenter and
@@ -470,7 +470,9 @@
     raw_input: Option<LinuxXInputController>,
     window: sdl2::video::Window,
     video: sdl2::VideoSubsystem,
-    event_pump: sdl2::EventPump,
+    // SDL allows one EventPump per process; release_event_pump hands it back so a fallback
+    // output can create its own while this one is still alive.
+    event_pump: Option<sdl2::EventPump>,
     audio: AudioDevice<StreamAudioCallback>,
     output: Arc<OutputBuffers>,
     external_renderer: bool,
@@ -483,7 +485,7 @@
     _sdl: sdl2::Sdl,
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl LinuxHardwareOutput {
     fn initialize(
         output: Arc<OutputBuffers>,
@@ -562,10 +564,14 @@
             None
         };
         let external_relative_motion = raw_input.is_some();
+        // The XInput2 thread also delivers buttons and the wheel, but only while relative mouse
+        // mode is on (see SdlInputCapture::external_mouse_buttons_active). If it failed to
+        // start, SDL keeps every event.
+        let external_mouse_buttons = raw_input.is_some();
         let mut input_capture = SdlInputCapture::new(
             capture_input,
             external_relative_motion,
-            false,
+            external_mouse_buttons,
             stream.shortcuts,
         );
         input_capture.enable_gamepads(&sdl);
@@ -576,7 +582,7 @@
             raw_input,
             window,
             video,
-            event_pump,
+            event_pump: Some(event_pump),
             audio,
             output,
             external_renderer,
@@ -619,6 +625,10 @@
         }
     }
 
+    fn release_event_pump(&mut self) {
+        self.event_pump = None;
+    }
+
     fn stop(&mut self) {
         self.input_capture.release(&self._sdl, &mut self.window);
         if let Some(raw_input) = self.raw_input.as_ref() {
@@ -728,7 +738,12 @@
         {
             surface.refresh_ordering()?;
         }
-        for event in self.event_pump.poll_iter().collect::<Vec<_>>() {
+        let events = self
+            .event_pump
+            .as_mut()
+            .map(|pump| pump.poll_iter().collect::<Vec<_>>())
+            .unwrap_or_default();
+        for event in events {
             if matches!(event, sdl2::event::Event::Quit { .. }) {
                 self.input_capture.release(&self._sdl, &mut self.window);
                 if let Ok(surface) = self.native_surface.as_mut() {
@@ -841,14 +856,14 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 impl Drop for LinuxHardwareOutput {
     fn drop(&mut self) {
         self.stop();
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn create_linux_presenter(
     window: &sdl2::video::Window,
     width: u32,
@@ -903,7 +918,7 @@
     .map_err(|error| error.to_string())
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn configure_linux_sdl_video_driver() {
     let native_wayland = std::env::var("OPENNOW_LINUX_NATIVE_WAYLAND")
         .ok()
@@ -928,7 +943,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 fn linux_display_refresh_hz(
     video: &sdl2::VideoSubsystem,
     window: &sdl2::video::Window,
@@ -1024,6 +1039,15 @@
         }
     }
 
+    /// Whether an external raw-input thread currently owns mouse buttons and the wheel, so SDL
+    /// must not report them too. The Windows raw input controller always does. The Linux
+    /// XInput2 thread only captures while relative mouse mode is on, so SDL keeps ownership in
+    /// absolute mode and whenever that thread did not start.
+    fn external_mouse_buttons_active(&self) -> bool {
+        self.external_mouse_buttons
+            && (!cfg!(any(target_os = "linux", target_os = "freebsd")) || self.relative_mouse)
+    }
+
     pub(crate) fn enable_gamepads(&mut self, sdl: &sdl2::Sdl) {
         let Ok(subsystem) = sdl.game_controller() else {
             return;
@@ -1288,7 +1312,7 @@
                 if self.cursor_state == RemoteCursorState::Hidden {
                     self.enable_relative_mouse(sdl, window);
                 }
-                if !self.external_mouse_buttons
+                if !self.external_mouse_buttons_active()
                     && let Some(button) = sdl_mouse_button(mouse_btn)
                     && self.pressed_buttons.insert(button)
                 {
@@ -1299,8 +1323,9 @@
                 }
             }
             Event::MouseButtonUp { mouse_btn, .. } => {
-                if !self.external_mouse_buttons
-                    && let Some(button) = sdl_mouse_button(mouse_btn)
+                // A press SDL reported is always released by SDL, even if the external thread
+                // took ownership while the button was held.
+                if let Some(button) = sdl_mouse_button(mouse_btn)
                     && self.pressed_buttons.remove(&button)
                 {
                     self.captured.push(CapturedInput::MouseButton {
@@ -1311,7 +1336,7 @@
             }
             Event::MouseWheel {
                 x, y, direction, ..
-            } if !self.external_mouse_buttons && (x != 0 || y != 0) => {
+            } if !self.external_mouse_buttons_active() && (x != 0 || y != 0) => {
                 let direction = if direction == sdl2::mouse::MouseWheelDirection::Flipped {
                     -1
                 } else {
@@ -1490,7 +1515,7 @@
         std::mem::take(&mut self.captured)
     }
 
-    #[cfg(any(target_os = "linux", target_os = "windows"))]
+    #[cfg(any(any(target_os = "linux", target_os = "freebsd"), target_os = "windows"))]
     pub(crate) const fn relative_mouse_enabled(&self) -> bool {
         self.relative_mouse
     }
@@ -1549,7 +1574,7 @@
     texture: Option<Texture>,
     native_surface: Result<NativeSurface, String>,
     input_capture: SdlInputCapture,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     raw_input: Option<LinuxXInputController>,
     canvas: WindowCanvas,
     texture_size: Option<(u32, u32)>,
@@ -1558,7 +1583,7 @@
     audio: AudioDevice<StreamAudioCallback>,
     output: Arc<OutputBuffers>,
     external_renderer: bool,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     presented_linux_frame: bool,
     visible: bool,
     paused: bool,
@@ -1574,7 +1599,7 @@
         if crate::HARDWARE_ONLY {
             return Err(crate::HARDWARE_ONLY_ERROR.to_owned());
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         configure_linux_sdl_video_driver();
         let sdl = sdl2::init().map_err(|error| format!("SDL initialization failed: {error}"))?;
         let video = sdl
@@ -1590,7 +1615,7 @@
             .resizable()
             .hidden()
             .metal_view();
-        if external_renderer && cfg!(target_os = "linux") {
+        if external_renderer && cfg!(any(target_os = "linux", target_os = "freebsd")) {
             window_builder.allow_highdpi();
         }
         if !external_renderer {
@@ -1623,10 +1648,10 @@
         } else {
             NativeSurface::new(canvas.window())
         };
-        let capture_input = cfg!(any(target_os = "windows", target_os = "linux"))
+        let capture_input = cfg!(any(target_os = "windows", any(target_os = "linux", target_os = "freebsd")))
             && external_renderer
             && native_input_capture_enabled();
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         let raw_input = if capture_input && video.current_video_driver() == "x11" {
             match LinuxXInputController::start(output.captured_input()) {
                 Ok(controller) => Some(controller),
@@ -1640,14 +1665,15 @@
         } else {
             None
         };
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         let external_relative_motion = raw_input.is_some();
-        #[cfg(not(target_os = "linux"))]
+        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
         let external_relative_motion = false;
+        let external_mouse_buttons = external_relative_motion;
         let mut input_capture = SdlInputCapture::new(
             capture_input,
             external_relative_motion,
-            false,
+            external_mouse_buttons,
             stream.shortcuts,
         );
         input_capture.enable_gamepads(&sdl);
@@ -1656,7 +1682,7 @@
             texture: None,
             native_surface,
             input_capture,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             raw_input,
             canvas,
             texture_size: None,
@@ -1665,7 +1691,7 @@
             audio,
             output,
             external_renderer,
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             presented_linux_frame: false,
             visible: false,
             paused: false,
@@ -1675,7 +1701,7 @@
 
     fn start(&mut self, surface: Option<&RenderSurface>) -> Result<(), String> {
         self.paused = false;
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             self.presented_linux_frame = false;
         }
@@ -1692,7 +1718,7 @@
         if paused {
             self.input_capture
                 .set_input_paused(true, &self._sdl, self.canvas.window_mut());
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             if let Some(raw_input) = self.raw_input.as_ref() {
                 raw_input.set_enabled(false);
             }
@@ -1708,7 +1734,7 @@
     fn stop(&mut self) {
         self.input_capture
             .release(&self._sdl, self.canvas.window_mut());
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Some(raw_input) = self.raw_input.as_ref() {
             raw_input.set_enabled(false);
         }
@@ -1726,7 +1752,7 @@
         self.texture = None;
         self.texture_size = None;
         self.texture_format = None;
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             self.presented_linux_frame = false;
         }
@@ -1734,7 +1760,7 @@
 
     fn update_surface(&mut self, surface: &RenderSurface) -> Result<(), String> {
         let Some(rect) = surface.rect.filter(|_| surface.visible) else {
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             if let Some(raw_input) = self.raw_input.as_ref() {
                 raw_input.set_enabled(false);
             }
@@ -1807,7 +1833,7 @@
                 event,
             );
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Some(raw_input) = self.raw_input.as_ref() {
             raw_input.set_enabled(
                 self.visible
@@ -1817,18 +1843,18 @@
         }
         if self.paused || !self.visible {
             self.output.take_video();
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             self.output.clear_linux_video();
             return Ok(false);
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if let Some(frame) = self.output.take_linux_video() {
             return self.present_linux_frame(frame.frame);
         }
         let Some(frame) = self.output.take_video() else {
             return Ok(false);
         };
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             self.presented_linux_frame = false;
         }
@@ -1857,7 +1883,7 @@
         Ok(true)
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     fn present_linux_frame(&mut self, frame: LinuxDecodedVideoFrame) -> Result<bool, String> {
         use opennow_streamer_platform_linux::PixelFormat;
 
@@ -1921,7 +1947,7 @@
     }
 
     fn backend_label(&self) -> &'static str {
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if self.presented_linux_frame {
             return "Linux decoder/SDL NV12";
         }
@@ -2325,7 +2351,7 @@
     Software(Box<SoftwareOutput>),
     #[cfg(target_os = "windows")]
     Windows(Box<WindowsOutput>),
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     LinuxHardware(Box<LinuxHardwareOutput>),
     #[cfg(target_os = "macos")]
     Mac(Box<crate::macos_backend::MacOutput>),
@@ -2342,11 +2368,20 @@
         matches!(self, Self::Mac(_))
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     pub(crate) fn is_linux_hardware(&self) -> bool {
         matches!(self, Self::LinuxHardware(_))
     }
 
+    /// Gives back the SDL event pump of an output that is about to be replaced. SDL allows only
+    /// one per process, so a replacement created while this output is alive could not get its own.
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
+    pub(crate) fn release_event_pump(&mut self) {
+        if let Self::LinuxHardware(output) = self {
+            output.release_event_pump();
+        }
+    }
+
     #[cfg(target_os = "windows")]
     pub(crate) fn is_windows_hardware(&self) -> bool {
         matches!(self, Self::Windows(output) if output.is_hardware())
@@ -2387,7 +2422,7 @@
                 .map(Box::new)
                 .map(Self::Windows);
         }
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         if use_linux_hardware {
             return LinuxHardwareOutput::initialize(output, stream, audio_device)
                 .map(Box::new)
@@ -2405,7 +2440,7 @@
             Self::Software(output) => output.start(surface),
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.start(surface),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => output.start(surface),
             #[cfg(target_os = "macos")]
             Self::Mac(output) => output.start(surface),
@@ -2420,7 +2455,7 @@
             }
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.set_paused(paused),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => {
                 output.set_paused(paused);
                 Ok(())
@@ -2435,7 +2470,7 @@
             Self::Software(output) => output.stop(),
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.stop(),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => output.stop(),
             #[cfg(target_os = "macos")]
             Self::Mac(output) => output.stop(),
@@ -2447,7 +2482,7 @@
             Self::Software(output) => output.update_surface(surface),
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.update_surface(surface),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => output.update_surface(surface),
             #[cfg(target_os = "macos")]
             Self::Mac(output) => output.update_surface(surface),
@@ -2466,7 +2501,7 @@
             }
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.pump(),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => output.pump().map(|presented| {
                 if presented {
                     OutputEvent::Presented("Linux decoder/Vulkan")
@@ -2484,7 +2519,7 @@
             Self::Software(output) => output.take_captured_input(),
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.take_captured_input(),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => output.take_captured_input(),
             #[cfg(target_os = "macos")]
             Self::Mac(output) => output.take_captured_input(),
@@ -2496,7 +2531,7 @@
             Self::Software(output) => output.update_cursor(bytes),
             #[cfg(target_os = "windows")]
             Self::Windows(output) => output.update_cursor(bytes),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => output.update_cursor(bytes),
             #[cfg(target_os = "macos")]
             Self::Mac(output) => output.update_cursor(bytes),
@@ -2528,7 +2563,7 @@
                     }
                 }
             }
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             Self::LinuxHardware(output) => match control {
                 OutputControl::PointerLock => {
                     output
@@ -3146,7 +3181,7 @@
 
 #[cfg(test)]
 mod tests {
-    #[cfg(all(target_os = "linux", feature = "hardware-only"))]
+    #[cfg(all(any(target_os = "linux", target_os = "freebsd"), feature = "hardware-only"))]
     #[test]
     fn hardware_only_rejects_sdl_fallback_before_initialization() {
         let result = super::SoftwareOutput::initialize(
@@ -3260,7 +3295,7 @@
     fn audio_buffer_drops_oldest_samples() {
         let output = OutputBuffers {
             video: Mutex::new(None),
-            #[cfg(target_os = "linux")]
+            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
             linux_video: Mutex::new(VecDeque::with_capacity(LINUX_VIDEO_QUEUE_CAPACITY)),
             audio: Mutex::new(AudioPlayoutBuffer::new(4)),
             audio_push_barrier: Mutex::new(None),
@@ -3292,7 +3327,7 @@
         assert_eq!(output.take_video().expect("frame").width, 2);
     }
 
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     #[test]
     fn linux_video_queue_smooths_bursts_and_bounds_recovery_latency() {
         use opennow_streamer_platform_linux::StreamFormat;
@@ -3453,6 +3488,25 @@
     }
 
     #[test]
+    fn external_mouse_buttons_follow_the_raw_input_owner() {
+        let mut capture =
+            SdlInputCapture::new(true, true, true, StreamShortcutBindings::default());
+        // Windows raw input always owns buttons; the Linux XInput2 thread only in relative mode.
+        capture.relative_mouse = false;
+        assert_eq!(
+            capture.external_mouse_buttons_active(),
+            !cfg!(any(target_os = "linux", target_os = "freebsd"))
+        );
+        capture.relative_mouse = true;
+        assert!(capture.external_mouse_buttons_active());
+        // Without an external owner SDL keeps the buttons in every mode.
+        let mut capture =
+            SdlInputCapture::new(true, false, false, StreamShortcutBindings::default());
+        capture.relative_mouse = true;
+        assert!(!capture.external_mouse_buttons_active());
+    }
+
+    #[test]
     fn only_predefined_cursor_zero_enters_relative_mouse_mode() {
         assert_eq!(cursor_state_for_message(0, 0), RemoteCursorState::Hidden);
         assert_eq!(cursor_state_for_message(0, 1), RemoteCursorState::Visible);
