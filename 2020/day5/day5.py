import math
raw_data = []
with open('day5.input', 'r') as data:
    raw_data = [d.rstrip() for d in data.readlines()]


def binaryStep(borders, upper):
    border = (borders[0] + borders[1]) / 2
    if upper:
        return (math.ceil(border), borders[1])
    else:
        return (borders[0], math.floor(border))

seatIds = []
max_seatId = -1
for d in raw_data:
    rows = (0, 127)
    cols = (0, 7)
    for letter in d:
        if letter in ('F', 'B'):
            rows = binaryStep(rows, letter == 'B')
        else:
             cols = binaryStep(cols, letter == 'R')
    seatId = rows[0] * 8 + cols[0]
    max_seatId = seatId if seatId > max_seatId else max_seatId
    seatIds.append(seatId)
    # print('{}: row {}, column {}, seat ID {}'.format(d, rows[0], cols[0], seatId))


def part1():
    print('Max seat ID: {}'.format(max_seatId))

def part2():
    ids = sorted(seatIds)
    for idx in range(1, len(seatIds)):
        if (ids[idx -1] +1 != ids[idx]):
            print('Missing seat: {}'.format(ids[idx-1] + 1))
            break

part1()
part2()