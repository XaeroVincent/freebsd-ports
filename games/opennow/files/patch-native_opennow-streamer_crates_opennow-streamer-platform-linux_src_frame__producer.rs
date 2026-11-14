--- native/opennow-streamer/crates/opennow-streamer-platform-linux/src/frame_producer.rs.orig
+++ native/opennow-streamer/crates/opennow-streamer-platform-linux/src/frame_producer.rs
@@ -817,8 +817,14 @@
             chroma_offset: chroma.offset,
             chroma_pitch: chroma.pitch,
         };
-        if let Some(imported) = self.imported.get(&key).and_then(Weak::upgrade) {
-            return Ok(imported);
+        // FreeBSD DMA-BUF descriptors carry no per-buffer inode, so (device, inode) cannot
+        // tell buffers apart there: the cache would hand back the first imported image for
+        // every later frame and the picture would freeze. Import each frame instead.
+        let cacheable = !cfg!(target_os = "freebsd");
+        if cacheable {
+            if let Some(imported) = self.imported.get(&key).and_then(Weak::upgrade) {
+                return Ok(imported);
+            }
         }
         self.imported
             .retain(|_, imported| imported.strong_count() != 0);
@@ -834,7 +840,9 @@
             chroma,
             source,
         )?);
-        self.imported.insert(key, Arc::downgrade(&imported));
+        if cacheable {
+            self.imported.insert(key, Arc::downgrade(&imported));
+        }
         Ok(imported)
     }
 
