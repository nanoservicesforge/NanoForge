#!/usr/bin/env python3
import os
import subprocess
import shutil
import tempfile
import platform


def main():
    # Create a temporary directory
    temp_dir = tempfile.mkdtemp()

    # Clone the repository
    clone_command = ["git", "clone", "https://github.com/nanoservicesforge/NanoForge.git", temp_dir]
    subprocess.run(clone_command, check=True)

    # Change directory to the cloned repository
    os.chdir(temp_dir)

    # Remove the .git folder
    shutil.rmtree(os.path.join(temp_dir, ".git"))

    # Compile the project
    subprocess.run(["cargo", "build", "--release"], check=True)

    # Check for OS and move the binary to the system's bin directory
    if platform.system() == "Linux":
        # Linux
        binary_path = os.path.join(temp_dir, "target", "release", "nanoforge")
        if os.path.exists("/usr/local/bin/nanoforge"):
            os.remove("/usr/local/bin/nanoforge")
        subprocess.run(["sudo", "mv", binary_path, "/usr/local/bin/"])

    elif platform.system() == "Darwin":
        # macOS
        binary_path = os.path.join(temp_dir, "target", "release", "nanoforge")
        if os.path.exists("/usr/local/bin/nanoforge"):
            os.remove("/usr/local/bin/nanoforge")
        subprocess.run(["sudo", "mv", binary_path, "/usr/local/bin/"])
    else:
        print("Unsupported OS")
        shutil.rmtree(temp_dir)
        exit(1)

    # Clean up
    shutil.rmtree(temp_dir)

    print("Installation complete.")


if __name__ == "__main__":
    main()
