import numpy as np
import regex as re


def move_crates(config, instructions, reverse):
    if not instructions:
        return config

    count, start_col, end_col = instructions.pop(0)
    moved_boxes = [config[start_col - 1].pop() for i in range(count)]
    # Reversing simulates "move a bunch of boxes in one grab"
    moved_boxes = reversed(moved_boxes) if reverse else moved_boxes
    config[end_col - 1] += moved_boxes

    return move_crates(config, instructions, reverse)


def get_metadata(f):
    # Get the contents. Break on newline to split config from instructions
    config, instructions = open(f, "r").read().rstrip().split("\n\n")
    config = config.split("\n")[:-1]

    def fix_row(row):
        # ChatGPT with a helping hand constructing this regex 😉
        regex = r" (   )|\[(.*?)\]"
        matches = re.findall(regex, row)
        return [i[1].strip() for i in matches]

    # Convert each row to a proper python.
    # e.g. "    [D]    " becomes ['', 'D', '']
    config = map(fix_row, config)
    # Rotate our rows 270º so we get our columns; useful for push/pop
    config = np.rot90(np.array(list(config)), k=3).tolist()
    # Clear out blank characters required to keep all columns the same size for rotation
    config = [[i for i in col if i] for col in config]

    instructions = [
        [int(s) for s in row.split() if s.isdigit()] for row in instructions.split("\n")
    ]

    return config, instructions


def run(f, reverse):
    config, instructions = get_metadata(f)
    config = move_crates(config, instructions, reverse)
    sol = "".join([i.pop() for i in config])
    return sol


sol = run("05/example.txt", reverse=False)
assert sol == "CMZ"
sol = run("05/example.txt", reverse=True)
assert sol == "MCD"

sol = run("05/input.txt", reverse=False)
print(f"The solution to the first problem is {sol}.")
sol = run("05/input.txt", reverse=True)
print(f"The solution to the second problem is {sol}.")
