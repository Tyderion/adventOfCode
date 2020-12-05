theMap = []
with open('day3.input','r') as data:
    theMap = [[char for char in list(d) if char != '\n'] for d in data.readlines()]


# Part 1
# Not very efficient if array was big
def part1():
    width = len(theMap[0])
    position_w = 0
    trees = 0
    for position_h in range(1, len(theMap)):
        position_w = (position_w + 3) % width
        current = theMap[position_h]
        if (current[position_w] == '#'):
            trees += 1
    print(trees)

def check_slope(right, down):
    width = len(theMap[0])
    position_w = 0
    trees = 0
    for position_h in range(down, len(theMap), down):
        position_w = (position_w + right) % width
        current = theMap[position_h]
        if (current[position_w] == '#'):
            trees += 1

    return trees

def part2():
    print(check_slope(3, 1) * check_slope(1, 1) * check_slope(5, 1) * check_slope(7, 1) * check_slope(1, 2))

part1()
part2()


