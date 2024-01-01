import subprocess
import os


def main():
    executable_path = os.path.join(os.path.dirname(__file__), 'build', 'beautiful_pytest')

    subprocess.run(executable_path)


if __name__ == "__main__":
    main()
