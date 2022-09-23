from codecs import ignore_errors
from time import sleep
import urllib.request, shutil, os, sys
from urllib.request import urlopen
from shutil import copyfileobj
import zipfile
import diff_lib

#  this file diffs the existing switch-package.zip against whatever the latest nightly or
#  beta is, depending on arguments, and produces a upgrade.zip for that version to this
#  version, as well as an deletions.json file with the files that should be deleted

if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv or len(sys.argv) != 2:
    if len(sys.argv) != 2:
        print("bad argument length!")
    print("make_diff.py <nightly/beta>")
    exit(0)

if not os.path.exists("artifacts/switch-package.zip"):
    exit("package zip not found!")

if sys.argv[1] == "nightly":
    release_type = "Nightlies"
    url = "https://github.com/HDR-Development/HDR-" + release_type + "/releases/latest/download/switch-package.zip"
elif sys.argv[1] == "beta":
    release_type = "Releases"
    url = "https://github.com/HDR-Development/HDR-" + release_type + "/releases/latest/download/switch-package.zip"
else:
    release_type = "direct_url"
    url = sys.argv[1]

print("type: " + release_type + ", getting latest from url: " + url)

urllib.request.urlretrieve(url, "switch-package-previous.zip")

diff_lib.create_diff("switch-package-previous.zip", "artifacts/switch-package.zip", "upgrade")

# move the stuff to artifacts folder
if os.path.exists("upgrade_artifacts"):
    shutil.rmtree("upgrade_artifacts")
os.mkdir("upgrade_artifacts")
shutil.move("upgrade.zip", "upgrade_artifacts")
shutil.move("deletions.json", "upgrade_artifacts")

