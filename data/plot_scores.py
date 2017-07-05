from matplotlib import pyplot as plt
from matplotlib.ticker import MultipleLocator
from pickle import load

import sys
sys.path.append('../drafts')
import coding


access = load(open('accessible.pkl', 'rb'))
score = load(open('score_4.pkl', 'rb'))

N = len(score)

fig, ax = plt.subplots(figsize=(10,10))

ax.hist(score,
        [.5+i for i in range(-25, 25)], label='all configs', align='mid')
ax.hist([x for (x, b) in zip(score, access) if b],
        [.5+i for i in range(-25, 25)], label='accessible', align='mid')
ax.xaxis.set_major_locator(MultipleLocator(2))
ax.set_xlabel('score')
ax.yaxis.set_major_locator(MultipleLocator(100000))
ax.set_ylabel('number of occurences')
ax.yaxis.get_major_formatter().set_powerlimits((4,4))
ax.set_title('Histogram of the 4-Awari scores')
ax.legend()
ax.grid(True, axis='y')

fig.tight_layout()
#plt.show()
fig.savefig('score_4_hist.png')
