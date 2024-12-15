import random
import math
n = max(2, min(1000, int(10 ** (random.random() * 3))))
c = max(1, min(1000000000000, int(10 ** (random.random() * 12))))
h = [0] * n
for i in range(n):
    h[i] = random.randint(1, 100)
for i in range(n - 1):
    h[i + 1] += h[i]

print(f"{n} {c}")
for hi in h:
    print(hi)
