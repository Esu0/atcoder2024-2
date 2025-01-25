import random
n = 10
print(n)
a = [random.randint(0, 100) for _ in range(n)]
a.sort()
for ai in a:
    print(ai)
