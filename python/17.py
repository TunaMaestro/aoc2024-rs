program = [2,4,1,1,7,5,4,0,0,3,1,6,5,5,3,0]
revprog = program[::-1]
produced = []

a = 0
while produced != revprog:
    a <<= 3
    goal = revprog[len(produced)]
    print(a)
    for iter_a in range(8):
        test_a = a | iter_a
        print(goal, test_a)

        if goal == ((test_a % 8) ^ 7 ^ (test_a >> ((test_a % 8) ^ 1))) % 8:
            print(a, test_a)
            produced.append(goal)
            a = test_a
            break
    else:
        print("Fail")
        break
        
def out(a):
    return ((a%8) ^ 1 ^ (a >> a%8 ^ 1) ^ 6   )% 8
