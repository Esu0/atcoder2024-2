import random

def random_graph(n, m):
    for _ in range(m):
        a = random.randint(1, n)
        b = random.randint(1, n - 1)
        if a == b:
            b = n
        if a > b:
            a, b = b, a
        print(f"{a} {b}")

n = 25000
m = random.randint(n // 2, n * 4)
q = 100000
print(f"{n} {m} {q}")
random_graph(n, m)

for _ in range(q):
    a = random.randint(1, n)
    b = random.randint(1, n - 1)
    if a == b:
        b = n
    if a > b:
        a, b = b, a
    print(f"{a} {b}")
