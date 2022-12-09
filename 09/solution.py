def get_instructions(f):
    instructions = [
        (i.strip().split()[0], int(i.strip().split()[1]))
        for i in open(f, "r").readlines()
    ]
    return instructions


def move_tail(head_position, tail_position):
    for i in [0, 1]:
        j = abs(i - 1)
        if head_position[i] - tail_position[i] == 2:
            tail_position[j] = head_position[j]
            tail_position[i] += 1
        elif head_position[i] - tail_position[i] == -2:
            tail_position[j] = head_position[j]
            tail_position[i] -= 1

    return tail_position


def update_position(visited, head_position, tail_position, instruction):
    direction, count = instruction

    x, y = head_position
    if direction == "U":
        new_x, new_y = x, y + count
    if direction == "D":
        new_x, new_y = x, y - count
    if direction == "R":
        new_x, new_y = x + count, y
    if direction == "L":
        new_x, new_y = x - count, y

    step = -1 if direction in ["D", "L"] else 1

    for i, n in enumerate([(x, new_x), (y, new_y)]):
        for _ in range(*n, step):
            head_position[i] += step
            tail_position = move_tail(head_position, tail_position)

            # Add a new row to our array
            while len(visited) <= tail_position[1]:
                row = [0 for _ in range(0, len(visited[0]))]
                visited.append(row)
            # Add a new column to our array
            while len(visited[0]) <= tail_position[0]:
                new_grid = []
                for j in visited:
                    new_grid.append([k for k in j] + [0])
                visited = new_grid

            visited[tail_position[1]][tail_position[0]] = 1

    return visited, head_position, tail_position


def run(f):
    instructions = get_instructions(f)
    # Set our starting point as square 500,500 of the grid, so it's easier to back out if necessary
    g = 100
    visited, head_position, tail_position = [[0]], [g, g], [g, g]
    while instructions:
        visited, head_position, tail_position = update_position(
            visited, head_position, tail_position, instructions.pop(0)
        )

    count = 0
    for i in visited:
        for j in i:
            count += j
    return count


sol = run("09/example.txt")
assert sol == 13

sol = run("09/input.txt")
print(sol)
