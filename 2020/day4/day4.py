import re
raw_data = []
with open('day4.input', 'r') as data:
    raw_data = [d for d in data.readlines()]

required_fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

passports = []
current_passport = {}

for raw in raw_data:
    if raw == '\n':
        passports.append(current_passport)
        current_passport = {}
        continue
    parts = raw.rstrip().split(' ')
    for part in parts:
        keyval = part.split(':')
        current_passport[keyval[0]] = keyval[1]
passports.append(current_passport)


def part1():
    def isValid(passport):
        return all(key in passport for key in required_fields)

    valid_count = 0
    for passport in passports:
        if (isValid(passport)):
            valid_count += 1

    print('part 1: {}'.format(valid_count))


def part2():
    def validateHgt(hgt):
        match = re.match('([0-9]+)(in|cm)', hgt)
        if (match is None):
            return False
        if (match.group(2) == 'in'):
            return int(match.group(1)) in range(59, 77)
        else:
            return int(match.group(1)) in range(150, 194)
    validations = {
        'byr': lambda byr: len(byr) == 4 and int(byr) in range(1920, 2003),
        'iyr': lambda iyr: len(iyr) == 4 and int(iyr) in range(2010, 2021),
        'eyr': lambda eyr: len(eyr) == 4 and int(eyr) in range(2020, 2031),
        'hgt': validateHgt,
        'hcl': lambda hcl: re.match('^#[0-9a-f]{6}$', hcl) is not None,
        'ecl': lambda ecl: ecl in ('amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'),
        'pid': lambda pid: re.match('^[0-9]{9}$', pid) is not None,
    }

    def isValid(passport):
        if not all(key in passport for key in required_fields):
            return False
        for key in required_fields:
            if not validations[key](passport[key]):
                # print('passport {} is not valid due to {}'.format(passport, key))
                return False
        return True

    valid_count = 0
    for passport in passports:
        if (isValid(passport)):
            valid_count += 1

    print('part 2: {}'.format(valid_count))


part1()
part2()
