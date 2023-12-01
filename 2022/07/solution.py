SPACE_REQUIRED = 30000000
TOT_CAPACITY = 70000000


class Directory:
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.children = []
        self.type = "directory"

    def __repr__(self):
        return str({i.name: i for i in self.children})

    def add_child(self, child):
        self.children.append(child)

    def get_child(self, name):
        for c in self.children:
            if c.name == name:
                return c

        raise Exception(f'Child with given name "{name}" does not exist.')

    def get_size(self):
        size = 0
        for c in self.children:
            if c.type == "file":
                size += c.size
            elif c.type == "directory":
                size += c.get_size()
        return size

    def get_root(self):
        if self.name == "/":
            return self
        return self.parent.get_root()


class File:
    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size
        self.type = "file"

    def __repr__(self):
        return str(self.size)


def update_filesystem(cwd: Directory, commands: list, incoming_item: str) -> Directory:
    item = commands.pop(0)

    if ".." in incoming_item:
        return update_filesystem(cwd.parent, commands, item)
    elif "cd" in incoming_item:
        new_cwd = incoming_item.split()[2].strip()
        new_directory = cwd.get_child(new_cwd)
        return update_filesystem(new_directory, commands, item)

    # Run an ls and save the results to our filesystem
    # Get the first item from the list
    while not item.startswith("$"):
        item = [i.strip() for i in item.split()]
        if item[0] == "dir":
            new_object = Directory(item[1], cwd)
        else:
            new_object = File(item[1], int(item[0]))

        cwd.add_child(new_object)

        if commands:
            item = commands.pop(0)
        # Once we've exhausted our commands we have a complete filesystem
        else:
            return cwd

    return update_filesystem(cwd, commands, item)


def sum_small_directories(directory):
    sum = 0
    for i in directory.children:
        if i.type == "directory":
            if i.get_size() < 100000:
                sum += i.get_size()
            sum += sum_small_directories(i)
    return sum


def get_fs(f):
    commands = open(f, "r").readlines()
    root_directory = Directory("/", None)
    commands.pop(0)
    item = commands.pop(0)
    node = update_filesystem(root_directory, commands, item)
    fs = node.get_root()
    return fs


fs = get_fs("07/example.txt")
total_size = fs.get_size()
assert total_size == 48381165
assert sum_small_directories(fs) == 95437

fs = get_fs("07/input.txt")
sol = sum_small_directories(fs)
print(f"The answer to question 1 is {sol}.")

"""
================== PROBLEM 2 =====================
"""


def get_eligible_directories(directory, ideal_size):
    candidates = []
    for i in directory.children:
        if i.type == "directory":
            if i.get_size() > ideal_size:
                candidates += [i.get_size()]
            candidates += get_eligible_directories(i, ideal_size)
    return candidates


def get_ideal_size(fs):
    return SPACE_REQUIRED - (TOT_CAPACITY - fs.get_size())


fs = get_fs("07/example.txt")
ideal_size = get_ideal_size(fs)
sol = min(get_eligible_directories(fs, ideal_size))
assert sol == 24933642

fs = get_fs("07/input.txt")
ideal_size = get_ideal_size(fs)
sol = min(get_eligible_directories(fs, ideal_size))
print(f"The answer to question 2 is {sol}.")
