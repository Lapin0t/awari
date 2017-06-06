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

def move(state, i):
    n, r = divmod(state[i], NPITS - 1)
    state[i] = 0
    for j in range(NPITS - 1):
        idx = (i + j + 1) % NPITS
        if j < r:
            state[idx] += n + 1
        else:
            state[idx] += n
    last = (i + r) % NPITS
    win = 0
    while last >= PPITS and 2 <= state[last] <= 3:
        win += state[last]
        state[last] = 0
        last -= 1
    return win

def change_player(state):
    state[:] = state[PPITS:] + state[:PPITS]


def disp_state(state):
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
