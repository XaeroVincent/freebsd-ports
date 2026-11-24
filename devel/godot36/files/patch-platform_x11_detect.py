--- platform/x11/detect.py.orig	2026-08-21 13:31:05.000000000 -0700
+++ platform/x11/detect.py	2026-11-23 00:07:51.641844000 -0800
@@ -390,7 +390,7 @@ def configure(env):
             env["speechd"] = False
             print("Warning: Speech Dispatcher development libraries not found. Disabling Text-to-Speech support.")
 
-    if platform.system() == "Linux":
+    if platform.system() in ["Linux","FreeBSD"]:
         env.Append(CPPDEFINES=["JOYDEV_ENABLED"])
         if env["udev"]:
             if os.system("pkg-config --exists libudev") == 0:  # 0 means found
