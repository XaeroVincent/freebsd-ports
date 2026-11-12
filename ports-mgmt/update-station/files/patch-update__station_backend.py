--- update_station/backend.py.orig	2026-11-07 05:58:14.000000000 -0800
+++ update_station/backend.py	2026-11-10 16:47:01.500746000 -0800
@@ -8,6 +8,7 @@ import datetime
 import requests
 import bectl
 import datetime
+import urllib.parse
 from gi.repository import Gtk
 from update_station.data import Data
 from subprocess import Popen, PIPE, call, run, CompletedProcess
@@ -168,9 +169,11 @@ def get_enabled_repo_url(repo_type: str, abi: str = No
     output = run_command(['pkg', '-vv'], env=env).stdout
     url = ''
     for line in output.splitlines():
-        if match := re.search(rf'url\s*:\s*"(https://pkg\.[^"]+/{repo_type})"', line):
+        if match := re.search(rf'url\s*:\s*"((?:pkg\+)?https?://[^"]+/{repo_type})"', line):
             url = match[1]
         elif re.search(r'enabled\s*:\s*yes', line) and url:
+            if url.startswith("pkg+"):
+                url = url[4:]
             return url
         elif line.strip() == '}':
             url = ''
@@ -192,7 +195,7 @@ def get_default_base_repo_url(abi: str = None) -> str:
 
     :param abi: Optional ABI string (e.g., "FreeBSD:15:amd64"). If None, uses the current ABI.
 
-    :return: The base repository URL (GhostBSD-base repository).
+    :return: The base repository URL (FreeBSD-base repository).
     """
     return get_enabled_repo_url('base', abi)
 
@@ -222,27 +225,27 @@ def get_current_version() -> str:
 
 def get_current_version() -> str:
     """
-    Get the full GhostBSD version currently installed on the system.
+    Get the full FreeBSD version currently installed on the system.
 
-    :return: The full version string (e.g., "25.02-R14.3p8").
+    :return: The full version string (e.g., "R14.3p8").
     """
-    result = run_command(['ghostbsd-version'])
+    result = run_command(['freebsd-version'])
     return result.stdout.strip()
 
 
 def get_version(new_abi: str = None) -> str:
     """
-    Get the full GhostBSD version available in the repository.
+    Get the full FreeBSD version available in the repository.
 
     pkg rquery exits non-zero both when the query fails and when the repository simply has no
     such package, so the two cannot be told apart and an empty string covers both.
 
     :param new_abi: Optional new ABI string (e.g., "FreeBSD:15:amd64"). If None, uses the current ABI.
 
-    :return: The full version string (e.g., "25.02-R14.3p8"), or an empty string on failure.
+    :return: The full version string (e.g., "R14.3p8"), or an empty string on failure.
     """
     env = {'ABI': new_abi, 'IGNORE_OSVERSION': 'yes'} if new_abi else None
-    result = run_command(['pkg', 'rquery', '%v', 'GhostBSD-runtime'], env=env)
+    result = run_command(['pkg', 'rquery', '%v', 'FreeBSD-runtime'], env=env)
     if result.returncode != 0:
         return ''
     return result.stdout.strip()
@@ -250,11 +253,11 @@ def get_installed_version() -> str:
 
 def get_installed_version() -> str:
     """
-    Get the GhostBSD version of the installed GhostBSD-runtime package.
-
-    :return: The full version string (e.g., "26.1-R15.0p13"), or an empty string on failure.
+    Get the FreeBSD version of the installed FreeBSD-runtime package.
+
+    :return: The full version string (e.g., "R15.0p13"), or an empty string on failure.
     """
-    result = run_command(['pkg', 'query', '%v', 'GhostBSD-runtime'])
+    result = run_command(['pkg', 'query', '%v', 'FreeBSD-runtime'])
     if result.returncode != 0:
         return ''
     return result.stdout.strip()
@@ -262,14 +265,14 @@ def parse_version(version: str) -> tuple:
 
 def parse_version(version: str) -> tuple:
     """
-    Parse a GhostBSD version string into its numeric components.
+    Parse a FreeBSD version string into its numeric components.
 
     The package version uses "." before the "R" because pkg only compares what follows the
     last "-", while /etc/version keeps the "-".
 
-    :param version: The version string (e.g., "26.1-R15.0p13", "26.3.R15.1p6").
+    :param version: The version string (e.g., "R15.0p13", "R15.1p6").
 
-    :return: A tuple of (ghostbsd_major, ghostbsd_minor, freebsd_major, freebsd_minor),
+    :return: A tuple of (freebsd_major, freebsd_minor),
              or an empty tuple if the string does not match the expected format.
     """
     match = re.fullmatch(r'(\d+)\.(\d+)[-.]R(\d+)\.(\d+)(?:(?:a|b|rc|p)\d+)?', version.strip())
@@ -280,16 +283,16 @@ def classify_upgrade(installed: str, remote: str) -> s
 
 def classify_upgrade(installed: str, remote: str) -> str:
     """
-    Classify the upgrade between the installed and the repository GhostBSD version.
+    Classify the upgrade between the installed and the repository FreeBSD version.
 
     Major upgrades (FreeBSD ABI change) are detected separately through .next_version,
     so this only distinguishes upgrades served by the current ABI repository.
 
-    :param installed: The installed version string (e.g., "26.1-R15.0p13").
-    :param remote: The repository version string (e.g., "26.2-R15.1p2").
+    :param installed: The installed version string (e.g., "R15.0p13").
+    :param remote: The repository version string (e.g., "R15.1p2").
 
     :return: 'minor' if the FreeBSD minor version changes, 'release' if only the
-             GhostBSD release changes, or 'none' for patch level and package updates.
+             FreeBSD release changes, or 'none' for patch level and package updates.
     """
     installed_parts = parse_version(installed)
     remote_parts = parse_version(remote)
@@ -352,7 +355,7 @@ def create_be(full_version: str, upgrade: bool = False
     """
     Create a boot environment.
 
-    :param full_version: The complete version string (e.g., "25.02-R14.3p8").
+    :param full_version: The complete version string (e.g., "R14.3p8").
     :param upgrade: If True, creates a boot environment for a system upgrade.
 
     :return: The boot environment name.
@@ -399,7 +402,7 @@ def get_packages_outside_be() -> list:
 
     :return: List of installed package names.
     """
-    packages = ['ports', 'GhostBSD-src', 'GhostBSD-src-sys']
+    packages = ['ports', 'FreeBSD-src', 'FreeBSD-src-sys']
     return [package for package in packages if run(['pkg', 'info', '-e', package], check=False).returncode == 0]
 
 
@@ -735,12 +738,22 @@ def repo_online() -> bool:
     """
     Check if the repository is online.
     """
-    server = list(filter(None, get_default_repo_url().split('/')))[1]
+    repo_url = get_default_repo_url()
+    if not repo_url:
+        return False
+    if repo_url.startswith("pkg+"):
+        repo_url = repo_url[4:]
     try:
+        parsed = urllib.parse.urlparse(repo_url)
+        server = parsed.hostname
+        if not server:
+            parts = list(filter(None, repo_url.split('/')))
+            server = parts[1] if len(parts) > 1 else repo_url
+
         s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
         s.settimeout(5)
         s.connect((server, 80))
-    except OSError:
+    except (OSError, IndexError, AttributeError):
         return False
     else:
         s.close()
@@ -752,7 +765,12 @@ def repository_is_syncing() -> bool:
     Check if the repository is syncing.
     :return: True if the repository is syncing else False.
     """
-    syncing_url = f'{get_default_repo_url()}/.syncing'
+    repo_url = get_default_repo_url()
+    if not repo_url:
+        return False
+    if repo_url.startswith("pkg+"):
+        repo_url = repo_url[4:]
+    syncing_url = f'{repo_url}/.syncing'
     try:
         response = requests.get(syncing_url, timeout=5)
         return response.status_code == 200
@@ -788,8 +806,8 @@ def set_package_base_config_file() -> CompletedProcess
 
 
 def set_package_base_config_file() -> CompletedProcess:
-    # /usr/local/etc/pkg/repos/GhostBSD.conf
-    config_path = '/usr/local/etc/pkg/repos/GhostBSD.conf'
+    # /usr/local/etc/pkg/repos/FreeBSD.conf
+    config_path = '/usr/local/etc/pkg/repos/FreeBSD.conf'
     return run_command(['cp', f'{config_path}.default', config_path])
 
 
@@ -803,18 +821,18 @@ def install_ghostbsd_pkgbase(mount_point: str) -> Comp
 
 def install_ghostbsd_pkgbase(mount_point: str) -> CompletedProcess:
     """
-    This function is used to install the GhostBSD-base package.
+    This function is used to install the FreeBSD-base package.
     :param mount_point: The mount point of the basepkg-test.
     """
-    return run_command(['pkg-static', '-r', mount_point, 'install', '-y', '-r', 'GhostBSD-base', '-g', 'GhostBSD-*'])
+    return run_command(['pkg-static', '-r', mount_point, 'install', '-y', '-r', 'FreeBSD-base', '-g', 'FreeBSD-*'])
 
 
 def fetch_ghostbsd_pkgbase(mount_point: str) -> CompletedProcess:
     """
-    This function is used to download the GhostBSD-base package.
+    This function is used to download the FreeBSD-base package.
     :param mount_point: The mount point of the basepkg-test.
     """
-    return run_command(['pkg-static', '-r', mount_point, 'fetch', '-y', '-r', 'GhostBSD-base', '-g', 'GhostBSD-*'])
+    return run_command(['pkg-static', '-r', mount_point, 'fetch', '-y', '-r', 'FreeBSD-base', '-g', 'FreeBSD-*'])
 
 
 def restore_vital_files(mount_point: str) -> None:
@@ -835,5 +853,5 @@ def remove_package_config() -> CompletedProcess:
     This function is used to remove the package config file.
     :return: The CompletedProcess object.
     """
-    config_path = '/usr/local/etc/pkg/repos/GhostBSD.conf'
+    config_path = '/usr/local/etc/pkg/repos/FreeBSD.conf'
     return run_command(['rm', config_path])
