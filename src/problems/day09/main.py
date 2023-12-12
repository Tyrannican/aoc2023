with open("input.txt") as f:
    histories = []
    for history in f.read().splitlines():
        histories.append([int(x) for x in history.split()])


def diffs(history):
    return [history[i] - history[i - 1] for i in range(1, len(history))]

def predict_next(history):
    n = [history[-1]]
    while True:
        history = diffs(history)
        if all([x == 0 for x in history]):
            break

        n.append(history[-1])

    return sum(n)

part_1 = 0

for history in histories:
    part_1 += predict_next(history)

print(part_1)
