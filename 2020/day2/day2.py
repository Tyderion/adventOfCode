
import re

policiesAndPasswords = []
with open('day2.input','r') as data:
    policiesAndPasswords = [d for d in data.readlines()]


# Part 1
# Not very efficient if array was big
def part1():
    correct_passwords = 0
    for p in policiesAndPasswords:
        match = re.search('([0-9]+)-([0-9]+) ([a-zA-Z]): (\w+)', p)
        amount = range(int(match.group(1)), int(match.group(2))+1)
        char = match.group(3)
        password = match.group(4)
        matches = re.findall(char, password)
        #print(amount, char, password, matches, len(matches) in amount)
        if (len(matches) in amount):
            correct_passwords += 1
    print("number of correct passwords: {}".format(correct_passwords))

# Part 1
# Not very efficient if array was big
def part2():
    correct_passwords = 0
    for p in policiesAndPasswords:
        match = re.search('([0-9]+)-([0-9]+) ([a-zA-Z]): (\w+)', p)
        first_pos = int(match.group(1)) - 1
        second_pos = int(match.group(2)) - 1
        char = match.group(3)
        password = match.group(4)
        #print(amount, char, password, matches, len(matches) in amount)
        if (password[first_pos] == char) !=  (password[second_pos] == char):
            correct_passwords += 1
    print("number of correct passwords: {}".format(correct_passwords))
                
part1()
part2()