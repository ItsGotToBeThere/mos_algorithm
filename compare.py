import sys

print(sys.argv[1])
print(sys.argv[2])

f1 = open(sys.argv[1], encoding="utf-8")
f2 = open(sys.argv[2], encoding="utf-16")

l1 = f1.read().split("\n")
l2 = f2.read().split("\n")

for i in range(len(l1)):
    if int(l1[i].strip())!=int(l2[i].strip()):
        print("line ", i)
        print("Left = ", l1[i], " Right = ", l2[i])
        exit()
print(True)


