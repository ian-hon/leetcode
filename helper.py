l = [(l if (l[0] == 0) else [[1, 0][i] for i in l]) for l in matrix]
print(sorted(list(set([(str(i), len([n for n in l if n == i])) for i in l])), key=lambda x: x[1])[-1][1])