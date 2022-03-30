from codecs import ignore_errors
from time import sleep
import urllib.request, shutil, os, sys
from urllib.request import urlopen
from shutil import copyfileobj
import zipfile
import diff_lib


if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv or len(sys.argv) != 2:
    if len(sys.argv) != 2:
        print("bad argument length!")
    print("make_diff.py <nightly/beta>")
    exit(0)

if not os.path.exists("artifacts/switch-package.zip"):
    exit("package zip not found!")

if sys.argv[1] == "nightly":
    release_type = "Nightlies"
else:
    release_type = "Releases"

url = "https://github.com/HDR-Development/HDR-" + release_type + "/releases/latest/download/switch-package.zip"
print("getting latest from url: " + url)

urllib.request.urlretrieve(url, "switch-package-previous.zip")

diff_lib.create_diff("switch-package-previous.zip", "artifacts/switch-package.zip", "upgrade")



