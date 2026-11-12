--- native/opennow-streamer/crates/opennow-streamer-platform/src/native_surface.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform/src/native_surface.rs
@@ -44,7 +44,7 @@
     }
 }
 
-#[cfg(any(target_os = "windows", target_os = "linux"))]
+#[cfg(any(target_os = "windows", any(target_os = "linux", target_os = "freebsd")))]
 fn parse_handle(value: &str) -> Result<usize, String> {
     let trimmed = value.trim();
     let parsed = if let Some(hex) = trimmed.strip_prefix("0x") {
@@ -58,7 +58,7 @@
         .ok_or_else(|| format!("invalid shell native window handle: {value}"))
 }
 
-#[cfg(any(target_os = "windows", target_os = "linux"))]
+#[cfg(any(target_os = "windows", any(target_os = "linux", target_os = "freebsd")))]
 fn physical_rect(rect: RenderSurfaceRect, _scale: f32) -> (i32, i32, u32, u32) {
     (rect.x, rect.y, rect.width.max(2), rect.height.max(2))
 }
@@ -275,7 +275,7 @@
     }
 }
 
-#[cfg(all(test, any(target_os = "windows", target_os = "linux")))]
+#[cfg(all(test, any(target_os = "windows", any(target_os = "linux", target_os = "freebsd"))))]
 mod tests {
     use super::*;
 
@@ -306,7 +306,7 @@
     }
 }
 
-#[cfg(target_os = "linux")]
+#[cfg(any(target_os = "linux", target_os = "freebsd"))]
 mod platform {
     use std::sync::atomic::{AtomicI32, Ordering};
 
@@ -539,7 +539,7 @@
     }
 }
 
-#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
+#[cfg(not(any(target_os = "windows", any(target_os = "linux", target_os = "freebsd"), target_os = "macos")))]
 mod platform {
     use super::*;
 
