--- native/opennow-streamer/crates/opennow-streamer-platform/src/embedded_input.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/embedded_input.rs
@@ -15,7 +15,7 @@
     queue: Arc<CapturedInputQueue>,
     active: AtomicBool,
     gamepads: Mutex<[Option<u16>; 4]>,
-    #[cfg(target_os = "linux")]
+    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
     raw: Mutex<Option<crate::linux_xinput::LinuxXInputController>>,
     #[cfg(target_os = "windows")]
     raw: Mutex<Option<crate::windows_raw_input::WindowsRawInputController>>,
@@ -27,7 +27,7 @@
             queue,
             active: AtomicBool::new(false),
             gamepads: Mutex::new([None; 4]),
-            #[cfg(any(target_os = "linux", target_os = "windows"))]
+            #[cfg(any(any(target_os = "linux", target_os = "freebsd"), target_os = "windows"))]
             raw: Mutex::new(None),
         }
     }
@@ -133,7 +133,7 @@
     }
 
     fn set_raw_capture(&self, active: bool, relative_mouse: bool, window_handle: usize) -> bool {
-        #[cfg(target_os = "linux")]
+        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
         {
             let raw_enabled = x11_raw_capture_enabled(active, relative_mouse, window_handle);
             let mut raw = self
@@ -185,7 +185,7 @@
             }
         }
 
-        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
+        #[cfg(not(any(any(target_os = "linux", target_os = "freebsd"), target_os = "windows")))]
         {
             let _ = (active, relative_mouse, window_handle);
             false
@@ -197,12 +197,12 @@
     }
 }
 
-#[cfg(any(target_os = "linux", test))]
+#[cfg(any(any(target_os = "linux", target_os = "freebsd"), test))]
 const fn x11_raw_capture_enabled(active: bool, relative_mouse: bool, x11_window: usize) -> bool {
     raw_capture_enabled(active, relative_mouse) && x11_window != 0
 }
 
-#[cfg(any(target_os = "linux", target_os = "windows", test))]
+#[cfg(any(any(target_os = "linux", target_os = "freebsd"), target_os = "windows", test))]
 const fn raw_capture_enabled(active: bool, relative_mouse: bool) -> bool {
     active && relative_mouse
 }
