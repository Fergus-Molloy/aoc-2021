inputs = [
    'be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe',
    'edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc',
    'fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg',
    'fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb',
    'aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea',
    'fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb',
    'dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe',
    'bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef',
    'egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb',
    'gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce',
]

def format(input):
    return list(map(lambda w: ''.join(sorted(w)), input))
outputs = list(map(format, [input.split('|')[1].strip().split() for input in inputs]))
inputs = list(map(format, [input.split('|')[0].strip().split() for input in inputs]))

code = []
mapping = {2: 1, 3: 7, 4: 4, 7: 8}
for input in inputs:
    temp = {}
    for word in input:
       if len(word) in mapping:
            temp[mapping[len(word)]] = word
    
    # Find 6
    for word in input:
        if len(word) == 6 and any(char not in word for char in temp[1]):
            temp[6] = word
            break
    # Find 0
    for word in input:
        if len(word) == 6 and any(char not in word for char in temp[4]) and word not in temp.values():
            temp[0] = word
            break
    
    # Find 9 after 6 and 0 with length 6
    for word in input:
        if len(word) == 6 and word not in temp.values():
            temp[9] = word
            break
    # Find 5
    for word in input:
        if len(word) == 5 and all(char in temp[6] for char in word):
            temp[5] = word
            break
    
    # Find 3
    for word in input:
        if len(word) == 5 and all(char in temp[9] for char in word) and word not in temp.values():
            temp[3] = word
            break
    
    # Find 2 after 3 and 5 with length 5
    for word in input:
        if len(word) == 5 and word not in temp.values():
            temp[2] = word
    
    print(temp)
    # Transform key-value to value-key
    code.append({v: k for k, v in temp.items()})
    total = 0
all_totals = []
for i, output in enumerate(outputs):
    all_totals += [int(''.join(map(str, [code[i][word] for word in output])))]
print(all_totals)
total = sum(all_totals)
print(f'Sum of the output values is {total}')
