def is_palindrome(input: str):
    return input == input[::-1]

def check_line(line: list, i: int, character: str):
    prev_char = line[i-1]
    curr_char = line[i]
    next_char = line[i+1]
    return prev_char == curr_char and curr_char == next_char and curr_char == character

palindrome_count = 0
total = 0
biggest = 0

vec = [[]]

with open("./symbole.txt") as f:
    for i, line in enumerate(f):
        line = line.strip()

        if is_palindrome(line):
            print(line)
            palindrome_count+=1

        for char in line:
            if len(vec) <= i:
                vec.append([char])
            else:
                vec[i].append(char)

    for i in range(1, len(vec)-1):
        prev_line = vec[i-1]
        line = vec[i]
        next_line = vec[i+1]
        for j in range(1, len(line)-1):
            character = line[j]
            ch1 = check_line(prev_line, j, character)
            ch2 = check_line(line, j, character)
            ch3 = check_line(next_line, j, character)

            if ch1 and ch2 and ch3:
                print(f"({i+1},{j+1})")

    for i in range(len(vec)):
        value = ""

        for j in range(len(vec[i])):
            character = vec[i][j]

            if character == 'o':
                value += "0"
            elif character == '+':
                value += "1"
            elif character == '*':
                value += "2"

        result = 0
        for c in value:
            result = result * 3 + int(c)

        total += result
        if result > biggest:
            biggest = result


print(f"Palindrome count: {palindrome_count}")
print(f"Biggest: {biggest}")
print(f"Total: {total}")
