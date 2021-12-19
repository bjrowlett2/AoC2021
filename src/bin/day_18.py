import re

snailfish_numbers = []
with open("inputs/day_18.txt") as stream:
    for line in stream.readlines():
        snailfish_numbers.append(line.strip())

def isdigit(ch):
    return ('0' <= ch) and (ch <= '9')

def explode(snailfish_number, base, regex_match):
    #print(f"Explode: {regex_match}")
    start = base + regex_match.start(0)
    finish = base + regex_match.end(0)

    prefix = snailfish_number[:start]
    suffix = snailfish_number[finish:]
    
    left_value = int(regex_match[1])
    for t in range(len(prefix)):
        i = len(prefix) - t - 1
        if isdigit(prefix[i]) and isdigit(prefix[i-1]):
            continue
        left_numbers = re.search(r"(\d+)", prefix[i:])
        if left_numbers:
            left_start = i + left_numbers.start(0)
            left_finish = i + left_numbers.end(0)

            left_prefix = prefix[:left_start]
            left_suffix = prefix[left_finish:]
            
            value = left_value + int(left_numbers[1])
            prefix = left_prefix + str(value) + left_suffix
            break

    right_value = int(regex_match[2])
    right_numbers = re.search(r"(\d+)", suffix)
    if right_numbers:
        right_start = right_numbers.start(0)
        right_finish = right_numbers.end(0)
        
        right_prefix = suffix[:right_start]
        right_suffix = suffix[right_finish:]
        
        value = right_value + int(right_numbers[1])
        suffix = right_prefix + str(value) + right_suffix

    return prefix + "0" + suffix

def split(snailfish_number, base, regex_match, value):
    #print(f"Split: {regex_match}")
    start = base + regex_match.start(0)
    finish = base + regex_match.end(0)

    prefix = snailfish_number[:start]
    suffix = snailfish_number[finish:]

    left_value = int(value / 2)
    right_value = int(value / 2) + int(value % 2)

    return f"{prefix}[{left_value},{right_value}]{suffix}"

def reduce(snailfish_number):
    level = 0
    for i, ch in enumerate(snailfish_number):
        if level == 4:
            explode_match = re.match(r"^\[(\d+)\,(\d+)\]", snailfish_number[i:])
            if explode_match:
                return explode(snailfish_number, i, explode_match)
        
        if ch == '[':
            level += 1
        elif ch == ']':
            level -= 1

    for i in range(len(snailfish_number)):
        split_match = re.match(r"(\d+)", snailfish_number[i:])
        if split_match:
            value = int(split_match[1])
            if value >= 10:
                return split(snailfish_number, i, split_match, value)

    return snailfish_number # Unchanged

def magnitude(snailfish_number):
    snailfish_number = re.sub(r"\[", "(3*", snailfish_number)
    snailfish_number = re.sub(r"\,", "+2*", snailfish_number)
    snailfish_number = re.sub(r"\]", ")", snailfish_number)
    return eval(snailfish_number)

def add(a, b):
    result = f"[{a},{b}]"
    while True:
        new = reduce(result)
        if new == result:
            break
        result = new
    return result


# Part 1
result = snailfish_numbers[0]
for i in range(1, len(snailfish_numbers)):
    result = add(result, snailfish_numbers[i])
print(f"Part 1: {magnitude(result)}")


# Part 2
max_magnitude = 0
for x in range(0, len(snailfish_numbers)):
    for y in range(x + 1, len(snailfish_numbers)):
        result1 = add(snailfish_numbers[x], snailfish_numbers[y])
        magnitude1 = magnitude(result1)
        if magnitude1 > max_magnitude:
            max_magnitude = magnitude1

        result2 = add(snailfish_numbers[y], snailfish_numbers[x])
        magnitude2 = magnitude(result2)
        if magnitude2 > max_magnitude:
            max_magnitude = magnitude2
print(f"Part 2: {max_magnitude}")
