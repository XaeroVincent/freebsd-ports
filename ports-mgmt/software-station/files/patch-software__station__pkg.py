--- software_station_pkg.py.orig	2026-10-08 17:42:56.000000000 -0700
+++ software_station_pkg.py	2026-11-10 14:31:19.516247000 -0800
@@ -43,6 +43,8 @@ def repository_is_syncing():
         encoding='utf-8'
     )
     pkg_url = raw_url.stdout.read().strip().split('"')[1]
+    if pkg_url.startswith("pkg+"):
+        pkg_url = pkg_url[4:]
     syncing_url = f'{pkg_url}/.syncing'
     return True if requests.get(syncing_url).status_code == 200 else False
 
