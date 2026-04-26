import sys,math
import random as rd

size = sys.argv[1]

n = 2 ** 10
freq = 500
uniq = 500

match size:
    case "s":
        n = 20
        freq = 5
        uniq = 5
    case "m":
        n = 2**10
        freq = 500
        uniq = 500
    case "l":
        n = 2**20
        freq = 30_000
        uniq = 30_000

print(n)

sqrt_n = math.ceil(math.sqrt(n))

array = [str(rd.randint(0,sqrt_n)) for _ in range(n)]

print(" ".join(array))

k = rd.randint(0,sqrt_n)
total_queries = freq + uniq
print(f"{total_queries} {k}")

for _ in range(freq):
    idx = rd.randint(0,sqrt_n)
    rng = sorted([rd.randint(0,n-1) for _ in range(2)])
    print(f"F {idx} {rng[0]} {rng[1]}")

for _ in range(uniq):
    rng = sorted([rd.randint(0,n-1) for _ in range(2)])
    print(f"U {rng[0]} {rng[1]}")

