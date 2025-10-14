import re
import argparse

# Set up argument parser
parser = argparse.ArgumentParser(description="Sort <details> blocks in a file by summary name.")
parser.add_argument("input_file", help="Path to the input Markdown file")
parser.add_argument("output_file", help="Path to the output Markdown file")
args = parser.parse_args()

# Read the input file
with open(args.input_file, "r", encoding="utf-8") as file:
    content = file.read()

# Match everything up to the first <details> (prefix), then all <details> blocks
prefix_match = re.split(r"(?=<details><summary>)", content, maxsplit=1)

if len(prefix_match) == 2:
    prefix, rest = prefix_match
else:
    prefix = ""
    rest = content  # no <details> blocks found, so no sorting necessary

# Find all <details> blocks
pattern = r"<details><summary>(.*?)</summary>(.*?)</details>"
matches: list[str] = re.findall(pattern, rest, re.DOTALL)

# Sort the matches by the summary text
sorted_matches = sorted(matches, key=lambda x: x[0])

# Reconstruct the sorted content
sorted_details = "\n\n".join(
    f"<details><summary>{summary.strip()}</summary>\n\n{body.strip()}\n\n</details>\n\n" for summary, body in sorted_matches
)

# Combine preserved prefix with sorted details
sorted_content = prefix + sorted_details

# Write to output file
with open(args.output_file, "w", encoding="utf-8") as file:
    file.write(sorted_content)

print(f"Sorting complete. Output saved to '{args.output_file}'.")
