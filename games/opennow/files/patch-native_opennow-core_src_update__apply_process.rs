--- native/opennow-core/src/update_apply/process.rs.orig
+++ native/opennow-core/src/update_apply/process.rs
@@ -217,6 +217,60 @@
     Ok(false)
 }
 
+#[cfg(target_os = "freebsd")]
+fn group_is_running(group: u32) -> Result<bool, String> {
+    let mut mib = [
+        libc::CTL_KERN,
+        libc::KERN_PROC,
+        libc::KERN_PROC_PGRP,
+        group as libc::c_int,
+    ];
+    let mut size = 0usize;
+    if unsafe {
+        libc::sysctl(
+            mib.as_mut_ptr(),
+            mib.len() as libc::c_uint,
+            std::ptr::null_mut(),
+            &mut size,
+            std::ptr::null_mut(),
+            0,
+        )
+    } < 0
+    {
+        return Err(std::io::Error::last_os_error().to_string());
+    }
+    if size == 0 {
+        return Ok(false);
+    }
+    // The group can gain members between the sizing call and the read.
+    let mut procs: Vec<libc::kinfo_proc> =
+        vec![unsafe { std::mem::zeroed() }; size / std::mem::size_of::<libc::kinfo_proc>() + 16];
+    size = procs.len() * std::mem::size_of::<libc::kinfo_proc>();
+    if unsafe {
+        libc::sysctl(
+            mib.as_mut_ptr(),
+            mib.len() as libc::c_uint,
+            procs.as_mut_ptr().cast(),
+            &mut size,
+            std::ptr::null_mut(),
+            0,
+        )
+    } < 0
+    {
+        return Err(std::io::Error::last_os_error().to_string());
+    }
+    for process in procs
+        .iter()
+        .take(size / std::mem::size_of::<libc::kinfo_proc>())
+        .filter(|process| process.ki_pid > 0)
+    {
+        if capture(process.ki_pid as u32)?.is_some() {
+            return Ok(true);
+        }
+    }
+    Ok(false)
+}
+
 #[cfg(windows)]
 struct Job(windows_sys::Win32::Foundation::HANDLE);
 
@@ -484,6 +538,77 @@
         boot_id: String::new(),
     }))
 }
+
+#[cfg(target_os = "freebsd")]
+fn capture(pid: u32) -> Result<Option<ProcessIdentity>, String> {
+    use std::os::unix::ffi::OsStrExt;
+    let mut mib = [
+        libc::CTL_KERN,
+        libc::KERN_PROC,
+        libc::KERN_PROC_PID,
+        pid as libc::c_int,
+    ];
+    let mut info: libc::kinfo_proc = unsafe { std::mem::zeroed() };
+    let mut size = std::mem::size_of::<libc::kinfo_proc>();
+    if unsafe {
+        libc::sysctl(
+            mib.as_mut_ptr(),
+            mib.len() as libc::c_uint,
+            (&mut info as *mut libc::kinfo_proc).cast(),
+            &mut size,
+            std::ptr::null_mut(),
+            0,
+        )
+    } < 0
+    {
+        return Err(std::io::Error::last_os_error().to_string());
+    }
+    // A pid that no longer exists yields an empty result, not an error.
+    if size == 0 || info.ki_pid as u32 != pid || info.ki_stat == libc::SZOMB as libc::c_char {
+        return Ok(None);
+    }
+    let mut path_mib = [
+        libc::CTL_KERN,
+        libc::KERN_PROC,
+        libc::KERN_PROC_PATHNAME,
+        pid as libc::c_int,
+    ];
+    let mut path = vec![0u8; libc::PATH_MAX as usize];
+    let mut path_size = path.len();
+    if unsafe {
+        libc::sysctl(
+            path_mib.as_mut_ptr(),
+            path_mib.len() as libc::c_uint,
+            path.as_mut_ptr().cast(),
+            &mut path_size,
+            std::ptr::null_mut(),
+            0,
+        )
+    } < 0
+    {
+        let error = std::io::Error::last_os_error();
+        if matches!(error.raw_os_error(), Some(libc::ESRCH) | Some(libc::ENOENT)) {
+            return Ok(None);
+        }
+        return Err(error.to_string());
+    }
+    let end = path[..path_size]
+        .iter()
+        .position(|byte| *byte == 0)
+        .unwrap_or(path_size);
+    if end == 0 {
+        return Ok(None);
+    }
+    let executable = std::fs::canonicalize(Path::new(std::ffi::OsStr::from_bytes(&path[..end])))
+        .map_err(|error| error.to_string())?;
+    Ok(Some(ProcessIdentity {
+        pid,
+        started: info.ki_start.tv_sec as u64 * 1_000_000 + info.ki_start.tv_usec as u64,
+        executable,
+        group: info.ki_pgid as u32,
+        boot_id: String::new(),
+    }))
+}
 
 #[cfg(windows)]
 fn capture(pid: u32) -> Result<Option<ProcessIdentity>, String> {
