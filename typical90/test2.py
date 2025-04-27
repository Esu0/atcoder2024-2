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

n = 10
m = 12
print(f"{n} {m}")
random_graph(n, m)