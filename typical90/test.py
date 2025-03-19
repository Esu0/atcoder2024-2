import random

r = random.randint(1, 7)
g = random.randint(1, 7)
b = random.randint(1, 7)
k = random.randint(1, r + g + b)
x = random.randint(0, min(r + g, k))
y = random.randint(0, min(k, g + b))
z = random.randint(0, min(k, b + r))

print(f"{r} {g} {b} {k}")
print(f"{x} {y} {z}")
