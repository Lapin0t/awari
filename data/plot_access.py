from pickle import load
from matplotlib import pyplot as plt
from matplotlib.ticker import LogFormatterSciNotation, MultipleLocator
from bisect import bisect

import sys
sys.path.append('../drafts')
from coding import enc_min, n_boards


access_raw = load(open('accessible.pkl', 'rb'))

codes = [enc_min(n+1, 8) for n in range(25)]
access = [0]*25
for (i, x) in enumerate(access_raw):
    if x:
        access[bisect(codes, i)] += 1

fig, ax = plt.subplots(figsize=(10,10))

ax.bar(range(25), [n_boards(n, 8) for n in range(25)], label='all configs')
ax.bar(range(25), [a for a in access], label='accessible')
ax.set_xlabel('seeds')
ax.set_ylabel('configurations')
ax.yaxis.set_major_locator(MultipleLocator(100000))
ax.xaxis.set_major_locator(MultipleLocator(1))
ax.yaxis.get_major_formatter().set_powerlimits((4,4))
ax.set_title('Number of accessible 4-Awari board configurations by seeds')
ax.legend()
ax.grid(True, axis='y')

fig.tight_layout()
#plt.show()
fig.savefig('accessible.png')
