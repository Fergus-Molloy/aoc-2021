from collections import Counter


def find(most):
    rows = open('inputs/day3').readlines()
    for i in range(12):
        c = Counter(r[i] for r in rows)
        rows = [
            r for r in rows
            if (r[i] == '1') != (c['1'] >= c['0']) ^ most
        ]
        if len(rows) == 1:
            return rows[0]


if __name__ == "__main__":
    print(int(find(True), 2))
    print(int(find(False), 2))
