import random

n = 1000000
print(n)
for _ in range(n):
    print(['R', 'G', 'B'][random.randint(0, 2)], end="")
print()
for _ in range(n):
    print(['R', 'G', 'B'][random.randint(0, 2)], end="")
print()
