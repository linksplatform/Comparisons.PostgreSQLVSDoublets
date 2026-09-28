#!/usr/bin/env python3
"""Export the pinned public Doublets source from an existing Git checkout."""
import argparse
import io
from pathlib import Path
import subprocess
import tarfile

REVISION = "b11f33b4080a7ef6b6d1c056c40bbf758d6cdd7e"
SETTINGS_REVISION = "f08ce8fc84f8ab7ccdcf9293b5566007305254c1"


def export(checkout, revision, destination):
    subprocess.run(["git", "-C", str(checkout), "cat-file", "-e", revision + "^{commit}"], check=True)
    archive = subprocess.check_output(["git", "-C", str(checkout), "archive", revision])
    destination.mkdir(parents=True, exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(destination, filter="data")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("checkout", type=Path, help="Existing Data.Doublets.Gql checkout with its pinned Settings submodule")
    args = parser.parse_args()
    destination = Path(__file__).resolve().parent / "source"
    if destination.exists():
        raise SystemExit("graphql/source already exists; use a fresh checkout/output directory.")
    # Verify both objects before creating output. No network calls or checkout modifications.
    for checkout, revision in [(args.checkout, REVISION), (args.checkout / "Settings", SETTINGS_REVISION)]:
        subprocess.run(["git", "-C", str(checkout), "cat-file", "-e", revision + "^{commit}"], check=True)
    export(args.checkout, REVISION, destination)
    export(args.checkout / "Settings", SETTINGS_REVISION, destination / "Settings")
    print(f"Prepared unmodified Doublets {REVISION} with Settings {SETTINGS_REVISION}")


if __name__ == "__main__":
    main()
