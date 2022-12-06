import numpy as np
import regex as re


def get_top_layer(config):
    return "".join([i.pop() for i in config])


def move_crate(config, count, start_col, end_col, reverse=False):
    moved_boxes = []
    start_col -= 1
    end_col -= 1
    for i in range(count):
        moved_boxes.append(config[start_col].pop())

    moved_boxes = reversed(moved_boxes) if reverse else moved_boxes

    for i in moved_boxes:
        config[end_col].append(i)

    return config


def get_metadata(f):
    # Get the contents. Break on newline to split movement instructions
    # from configuration
    config, instructions = open(f, "r").read().rstrip().split("\n\n")

    config = config.split("\n")[:-1]

    def fix_row(row):
        # ChatGPT with a helping hand constructing this regex 😉
        regex = r" (   )|\[(.*?)\]"

        matches = re.findall(regex, row)
        return [i[1].strip() for i in matches]

    config = list(map(fix_row, config))

    config = np.rot90(np.array(config), k=3).tolist()

    # Clear out our placeholder characters required to keep all columns the same size during the rotation
    config = [[i for i in col if i] for col in config]

    instructions = [
        [int(s) for s in row.split() if s.isdigit()] for row in instructions.split("\n")
    ]

    return config, instructions


def run(f, reverse):
    config, instructions = get_metadata(f)

    for i in instructions:
        config = move_crate(config, *i, reverse=reverse)

    sol = get_top_layer(config)
    return sol


sol = run("05/example.txt", reverse=False)
assert sol == "CMZ"

sol = run("05/example.txt", reverse=True)
assert sol == "MCD"

sol = run("05/input.txt", reverse=False)
print(f"The solution to the first problem is {sol}.")

sol = run("05/input.txt", reverse=True)
print(f"The solution to the first problem is {sol}.")
