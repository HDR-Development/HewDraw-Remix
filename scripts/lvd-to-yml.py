import os
import subprocess
import sys
import glob
import shutil

def run_example_exe(executable_path, input_directory, output_directory):
    # List all files in the directory
    files = glob.glob(input_directory + "**/normal*/param/*.lvd", recursive=True)

    # Run example.exe for each .lvd file
    for file in files:
        print("file: ", file)

        # construct the output path
        output_file_path = os.path.join(
            output_directory + "yml\\",
            os.path.splitext(file.replace(input_directory, ""))[0] + ".yml",
        )
        print("output_file_path: ", output_file_path)

        # make the output directory if it doesn't exist
        output_file_directory = output_file_path.replace(
            os.path.basename(output_file_path), ""
        )
        if not os.path.exists(output_file_directory):
            # Create a new directory because it does not exist
            os.makedirs(output_file_directory)
            print(f"Made a new directory: {output_file_directory}")

        # construct the final command
        command = [executable_path, file, output_file_path]
        print("command: ", command)

        # run the command
        try:
            subprocess.run(command, check=True)
            # print(f"Successfully ran for {file}")
        except subprocess.CalledProcessError as e:
            print(f"Error running for {file}: {e}")

    print(f"Zipping lvd files into lvd.zip at {output_directory}")
    shutil.make_archive(output_directory + "lvd", "zip", output_directory + "yml\\")
    print("Done!")

if __name__ == "__main__":
    # Check if both directory and executable paths are provided as command-line arguments
    if len(sys.argv) != 4:
        print(
            "Usage: python script.py <yamlve_exe_path> <input_directory_path>"
            " <output_directory_path>"
        )
        sys.exit(1)

    # Get executable and directory paths from command-line arguments
    yamlve_exe_path = sys.argv[1]
    input_directory_path = sys.argv[2]
    ouput_directory_path = sys.argv[3]

    # Check if the executable exists
    if not os.path.isfile(yamlve_exe_path):
        print(f"Error: Executable '{yamlve_exe_path}' does not exist.")
        sys.exit(1)

    # Check if the directory exists
    if not os.path.isdir(input_directory_path):
        print(f"Error: Directory '{input_directory_path}' does not exist.")
        sys.exit(1)

    # Check if the directory exists
    if not os.path.isdir(ouput_directory_path):
        print(f"Error: Directory '{ouput_directory_path}' does not exist.")
        sys.exit(1)

    run_example_exe(yamlve_exe_path, input_directory_path, ouput_directory_path)
