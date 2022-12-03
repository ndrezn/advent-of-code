f = open("1/input.txt", "r")

elves_count = []

index = 0
for i in f.readlines():
    if i == "\n":
        index += 1
        continue
    try:
        elves_count[index] += int(i)
    except IndexError:
        elves_count.append(int(i))

sol = max(elves_count)
print(sol)
