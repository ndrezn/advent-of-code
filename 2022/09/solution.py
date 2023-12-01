def get_instructions(f):
    instructions = [
        (i.strip().split()[0], int(i.strip().split()[1]))
        for i in open(f, "r").readlines()
    ]
    return instructions


def move_head(head, direction):
    sign = {"R": [0, 1], "L": [0, -1], "U": [1, 1], "D": [1, -1]}
    x_y, mult = sign[direction]
    head[x_y] = head[x_y] + (1 * mult)
    return head


def move_tail(head, tail):
    # I understand this can be simplified.
    # Maybe for another time.
    for i in [0, 1]:
        j = abs(i - 1)
        if tail[i] - head[i] == -2:
            tail[i] += 1
            if tail[j] > head[j]:
                tail[j] -= 1
            elif tail[j] < head[j]:
                tail[j] += 1

        elif tail[j] - head[j] == 2:
            tail[j] -= 1
            if tail[i] > head[i]:
                tail[i] -= 1
            elif tail[i] < head[i]:
                tail[i] += 1
    return tail


def read_instructions(instructions, rope_size):
    head = [0, 0]
    rope = [[0, 0] for _ in range(0, rope_size)]
    seen = {}
    for direction, count in instructions:
        for _ in range(count):
            new_rope = []
            head = move_head(head, direction)
            prev = head
            for knot in rope:
                prev = move_tail(prev, knot)
                new_rope.append(prev)
            rope = new_rope
            seen[str(prev)] = True  # Is this cheating? nahhhh
    return len(seen)


i = get_instructions("09/example.txt")
seen = read_instructions(i, 1)
assert seen == 13

i = get_instructions("09/input.txt")
seen = read_instructions(i, 1)
print(f"The answer for problem 1 is {seen}.")

i = get_instructions("09/example.txt")
seen = read_instructions(i, 9)
assert seen == 1

i = get_instructions("09/example_long.txt")
seen = read_instructions(i, 9)
assert seen == 36

i = get_instructions("09/input.txt")
seen = read_instructions(i, 9)
print(f"The answer for problem 2 is {seen}.")
