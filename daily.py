from bs4 import BeautifulSoup
import requests
import sys

import os

YEAR = 2023
try:
    day = int(sys.argv[1])
except IndexError:
    print("Please provide a date as an integer.")
    sys.exit(1)

website = requests.get(f"https://adventofcode.com/2022/day/{day}")

footer = """To play, please identify yourself via one of these services:
[GitHub] [Google] [Twitter] [Reddit] - [How Does Auth Work?]"""

soup = BeautifulSoup(website.content, "html.parser")


def split_prompt(string):
    # Split the string into a list of words
    if len(string) < 120:
        return string
    words = string.split()

    # Initialize an empty list to store the processed string
    processed_string = []

    # Initialize a variable to store the current line
    line = ""

    # Loop through each word in the list of words
    for word in words:
        # If the current line plus the next word would be longer than 120 characters, add the current line to the processed string and start a new line
        if len(line) + len(word) > 120:
            processed_string.append(line)
            line = ""

        # Add the word to the current line
        line += word + " "

    # Add the remaining line to the processed string
    processed_string.append(line)

    # Print the processed string
    return "\n".join(processed_string)


prompt = (
    soup.text.split(f"Day {day}:")[1]
    .replace(" ---", "\n\n")
    .replace(footer, "")
    .strip()
)

prompt = "\n".join([split_prompt(i) for i in prompt.split("\n")])

prompt = f'"""\n{prompt}\n"""'

day_name = str(day).zfill(2)
if not os.path.exists(f"{YEAR}/{day_name}"):
    os.makedirs(f"{YEAR}/{day_name}")

# create a empty text file
fp = open(f"{YEAR}/{day_name}/solution.py", "w")
fp.write(prompt)
fp.close()

with open(f"{YEAR}/{day_name}/input.txt", "w") as fp:
    pass

with open(f"{YEAR}/{day_name}/example.txt", "w") as fp:
    pass

print(
    f"You can get your input by visiting (you must log in): https://adventofcode.com/2022/day/{day}/input"
)
print(f"Result can be submitted at: https://adventofcode.com/2022/day/{day}")
