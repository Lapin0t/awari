from collections import deque
import time


PPITS = 6
NPITS = 12

def successors_i(state):
    for i, x in enumerate(state[:PPITS]):
        if x > 0:
            yield i

def succs(state):
    for i in successors_i(state):
        s = list(state)
        move(s, i)
        change_player(s)
        yield tuple(s)

def sow(state, i):
    n, r = divmod(state[i], NPITS - 1)
    state[i] = 0
    for j in range(NPITS - 1):
        idx = (i + j + 1) % NPITS
        if j < r:
            state[idx] += n + 1
        else:
            state[idx] += n
    if r == 0:
        return (i-1) % NPITS, n - 1
    else:
        return (i+r) % NPITS, n

def unsow(state, i, n):
    for r in range(1, NPITS):
        a = (i + r) % NPITS
        if a < PPITS and state[a] == 0:
            break
    else:
        raise ValueError
    for k in range(1, r):
        state[(i+k) % NPITS] -= n
    for k in range(r+1, NPITS+1):
        state[(i+k) % NPITS] -= n+1
    state[(i+r) % NPITS] += NPITS - r + (NPITS-1)*n
    return (i+r) % NPITS

def collect(last):
    win = 0
    for i in reversed(range(PPITS, last + 1)):
        if state[i] not in (2, 3):
            break
        win += state[i]
        state[i] = 0
    return win

def change_player(state):
    state[:] = state[PPITS:] + state[:PPITS]


def disp(state):
    him = '|'.join('%2d' % n for n in state[PPITS:][::-1])
    me = '|'.join('%2d' % n for n in state[:PPITS])
    print(him)
    print('--+--+--+--+--+--')
    print(me)


def new_state(pebbles=4):
    return [pebbles for _ in range(NPITS)]

def explore(pebbles=4, timeout=1.):
    q = deque([tuple(new_state(pebbles))])
    adj = {}
    st = time.time()
    while q and (time.time() - st < timeout):
        u = q.popleft()
        adj[u] = tuple(succs(u))
        for s in adj[u]:
            if not s in adj:
                q.append(s)
    return adj

def write_dot(adj, out):
    i = 0
    trans = {}
    def get(u):
        nonlocal i
        if u in trans:
            return trans[u]
        else:
            trans[u] = i
            i += 1
            return i-1
    with open(out, 'w') as s:
        s.write('digraph bla {\n')
        for u in adj:
            s.write('{0:d} [label={0:d}];\n'.format(get(u)))
            for v in adj[u]:
                s.write('{0:d} -> {1:d};\n'.format(get(u), get(v)))
        s.write('}\n')
