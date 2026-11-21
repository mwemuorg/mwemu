"""Install missing release fixtures without overwriting locally built samples."""

import shutil
import sys
from pathlib import Path
from tempfile import TemporaryDirectory
from urllib.request import urlopen
from zipfile import ZipFile

BUNDLE_URL = "https://github.com/mwemuorg/mwemu/releases/download/maps/test.zip"
PASSWORD = b"mwemuTestSystem"


def install_samples(directory):
    directory.mkdir(parents=True, exist_ok=True)
    cache = directory / ".samples.zip"
    with TemporaryDirectory(dir=directory) as scratch:
        temporary = Path(scratch) / "download"
        if not cache.is_file():
            print(f"Fetching samples from {BUNDLE_URL}", flush=True)
            with urlopen(BUNDLE_URL, timeout=60) as source, temporary.open("wb") as dest:
                shutil.copyfileobj(source, dest)
            with ZipFile(temporary) as bundle:
                bundle.setpassword(PASSWORD)
                corrupt = bundle.testzip()
                if corrupt is not None:
                    raise ValueError(f"Corrupt sample in release archive: {corrupt}")
            temporary.replace(cache)

        restored = 0
        with ZipFile(cache) as bundle:
            for member in bundle.infolist():
                if member.is_dir():
                    continue
                relative = Path(member.filename).relative_to("test")
                dest = directory / relative
                if not dest.resolve().is_relative_to(directory.resolve()):
                    raise ValueError(f"Unexpected sample path: {member.filename}")
                if dest.is_file() and dest.stat().st_size:
                    continue
                dest.parent.mkdir(parents=True, exist_ok=True)
                with bundle.open(member, pwd=PASSWORD) as source, temporary.open("wb") as out:
                    shutil.copyfileobj(source, out)
                temporary.replace(dest)
                restored += 1
        print(f"Restored {restored} sample files in {directory}")


if __name__ == "__main__":
    install_samples(Path(sys.argv[1]))
