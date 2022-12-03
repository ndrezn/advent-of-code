outcomes = {
    "A": {
        "X": "C",
        "Z": "B",
    },
    "B": {"X": "A", "Z": "C"},
    "C": {"X": "B", "Z": "A"},
}


def get_move(a, b):
    if b == "Y":
        return a
    else:
        return outcomes[a][b]


def run(f):
    shape_score = {"A": 1, "B": 2, "C": 3}
    outcome_score = {"X": 0, "Y": 3, "Z": 6}

    score = 0
    for i in f.readlines():
        a, b = i.strip().split(" ")

        cur_score = outcome_score[b]

        move = get_move(a, b)

        cur_score += shape_score[move]

        score += cur_score
    return score


def test_base_case():
    f = open("2/example.txt", "r")
    score = run(f)
    print(score)


def test_example():
    f = open("2/input.txt", "r")
    score = run(f)
    print(score)


test_base_case()
test_example()
