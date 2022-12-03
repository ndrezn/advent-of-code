"""
--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
"""

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
