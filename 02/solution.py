def get_winner(a, b):
    if a == b:
        outcome = "draw"
    elif (a == "X" and b == "Z") or (a == "Y" and b == "X") or (a == "Z" and b == "Y"):
        outcome = "loss"
    else:
        outcome = "win"
    return outcome


def run(f):
    replacements = {"A": "X", "B": "Y", "C": "Z"}
    shape_score = {"X": 1, "Y": 2, "Z": 3}
    outcome_score = {"draw": 3, "loss": 0, "win": 6}

    score = 0
    for i in f.readlines():
        a, b = i.strip().split(" ")
        cur_score = shape_score[b]
        a = replacements[a]

        outcome = get_winner(a, b)

        cur_score += outcome_score[outcome]

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
