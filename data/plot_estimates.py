from matplotlib import pyplot as plt

def read_data(f='estimates'):
    data = {i: [] for i in range(2, 7)}
    with open(f) as s:
        for l in s.readlines():
            p, s, est, real = map(float, l.split())
            if real == 0:
                real = float('nan')
            data[p].append((s, est, real))
    for p in data:
        data[p] = tuple(zip(*sorted(data[p])))
    return data


def plot(data):
    fig, ax = plt.subplots()
    for (cl, (p, (a, b, c))) in zip('bgmcryk', data.items()):
        ax.plot(a, b, '-'+cl, alpha=.3)
        ax.plot(a, c, '-'+cl, label='%d pits' % p)
    ax.set_yscale('log')
    ax.legend()
