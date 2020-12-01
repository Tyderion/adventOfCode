
numbers = []
with open('day1.input','r') as data:
    numbers = [int(d) for d in data.readlines()]

# Part 1
# Not very efficient if array was big
def part1():
    for number in numbers:
        for number2 in numbers:
            if (number + number2) == 2020:
                print('{} + {} = 2020, {} * {} = {}'.format(number, number2, number, number2, number * number2))
                return

# Part 2
# very inefficient if array was big
def part2():
    for number in numbers:
        for number2 in numbers:
            for number3 in numbers:
                if (number + number2 + number3) == 2020:
                    print('{} + {} + {}= 2020, {} * {} * {} = {}'.format(number, number2, number3, number, number2, number3, number * number2 * number3))
                    return
                
part1()
part2()