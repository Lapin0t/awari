===================
Complexity analysis
===================

The goal here is to conduct a space and time complexity analysis of the
retrograde analysis algorithms for 2 player games.


Some data on the game of awari
==============================

Stats are given for each number of seeds on the board. The space is computed as
the theoretical minimum: N_CONFIGS*log(2*MAX_SEEDS+1, 8) bytes. "(a)" is for
the subset accessible during normal play and "(c)" is for cumulative values.


8 pits (4-awari)
----------------

=== ======== ============ ========== ========== ============
  n  configs  configs (a)      space  space (c)  space (a,c)
=== ======== ============ ========== ========== ============
  0        1            0    0.7 B      0.7 B        0.0 B  
  1        8            4    5.6 B      6.3 B        2.8 B  
  2       36           25   25.3 B     31.6 B       20.4 B  
  3      120           93   84.2 B    115.8 B       85.6 B  
  4      330          275  231.6 B    347.4 B      278.6 B  
  5      792          684  555.9 B    903.3 B      758.7 B  
  6     1716         1503    1.2 KiB    2.1 KiB      1.8 KiB
  7     3432         3017    2.4 KiB    4.4 KiB      3.8 KiB
  8     6435         5628    4.4 KiB    8.8 KiB      7.7 KiB
  9    11440         9898    7.8 KiB   16.7 KiB     14.5 KiB
 10    19448        16567   13.3 KiB   30.0 KiB     25.8 KiB
 11    31824        26586   21.8 KiB   51.8 KiB     44.1 KiB
 12    50388        41174   34.5 KiB   86.3 KiB     72.3 KiB
 13    77520        61838   53.1 KiB  139.5 KiB    114.7 KiB
 14   116280        90412   79.7 KiB  219.2 KiB    176.6 KiB
 15   170544       129081  116.9 KiB  336.1 KiB    265.1 KiB
 16   245157       180431  168.0 KiB  504.1 KiB    388.8 KiB
 17   346104       247385  237.2 KiB  741.3 KiB    558.3 KiB
 18   480700       333818  329.5 KiB    1.0 MiB    787.1 KiB
 19   657800       443180  450.8 KiB    1.5 MiB      1.1 MiB
 20   888030       578460  608.6 KiB    2.1 MiB      1.5 MiB
 21  1184040       736093  811.5 KiB    2.9 MiB      1.9 MiB
 22  1560780       951020    1.0 MiB    3.9 MiB      2.6 MiB
 23  2035800            0    1.4 MiB    5.3 MiB      2.6 MiB
 24  2629575      1349905    1.8 MiB    7.0 MiB      3.5 MiB
=== ======== ============ ========== ========== ============


12 pits (6-awari)
-----------------

=== ============= ========== ========== =============
  n       configs      space  space (c)  space (real)
=== ============= ========== ========== =============
  0             1    0.8 B      0.8 B         2.0 B  
  1            12    9.9 B     10.7 B        24.0 B  
  2            78   64.3 B     75.1 B       156.0 B  
  3           364  300.3 B    375.4 B       728.0 B  
  4          1365    1.1 KiB    1.5 KiB       2.7 KiB
  5          4368    3.5 KiB    5.0 KiB       8.5 KiB
  6         12376   10.0 KiB   15.0 KiB      24.2 KiB
  7         31824   25.6 KiB   40.6 KiB      62.2 KiB
  8         75582   60.9 KiB  101.5 KiB     147.6 KiB
  9        167960  135.3 KiB  236.8 KiB     328.0 KiB
 10        352716  284.2 KiB  521.0 KiB     688.9 KiB
 11        705432  568.3 KiB    1.1 MiB       1.3 MiB
 12       1352078    1.1 MiB    2.1 MiB       2.6 MiB
 13       2496144    2.0 MiB    4.1 MiB       4.8 MiB
 14       4457400    3.5 MiB    7.6 MiB       8.5 MiB
 15       7726160    6.1 MiB   13.7 MiB      14.7 MiB
 16      13037895   10.3 MiB   23.9 MiB      24.9 MiB
 17      21474180   16.9 MiB   40.8 MiB      41.0 MiB
 18      34597290   27.2 MiB   68.1 MiB      66.0 MiB
 19      54627300   43.0 MiB  111.0 MiB     104.2 MiB
 20      84672315   66.6 MiB  177.6 MiB     161.5 MiB
 21     129024480  101.5 MiB  279.2 MiB     246.1 MiB
 22     193536720  152.3 MiB  431.4 MiB     369.1 MiB
 23     286097760  225.1 MiB  656.5 MiB     545.7 MiB
 24     417225900  328.3 MiB  984.8 MiB     795.8 MiB
 25     600805296  472.7 MiB    1.4 GiB       1.1 GiB
 26     854992152  672.7 MiB    2.1 GiB       1.6 GiB
 27    1203322288  946.7 MiB    3.0 GiB       2.2 GiB
 28    1676056044    1.3 GiB    4.3 GiB       3.1 GiB
 29    2311801440    1.8 GiB    6.1 GiB       4.3 GiB
 30    3159461968    2.4 GiB    8.5 GiB       5.9 GiB
 31    4280561376    3.3 GiB   11.8 GiB       8.0 GiB
 32    5752004349    4.4 GiB   16.2 GiB      10.7 GiB
 33    7669339132    5.9 GiB   22.1 GiB      14.3 GiB
 34   10150595910    7.8 GiB   29.9 GiB      18.9 GiB
 35   13340783196   10.3 GiB   40.1 GiB      24.8 GiB
 36   17417133617   13.4 GiB   53.5 GiB      32.4 GiB
 37   22595200368   17.4 GiB   70.9 GiB      42.1 GiB
 38   29135916264   22.4 GiB   93.3 GiB      54.3 GiB
 39   37353738800   28.7 GiB  122.0 GiB      69.6 GiB
 40   47626016970   36.6 GiB  158.6 GiB      88.7 GiB
 41   60403728840   46.4 GiB  205.0 GiB     112.5 GiB
 42   76223753060   58.6 GiB  263.5 GiB     142.0 GiB
 43   95722852680   73.5 GiB  337.1 GiB     178.3 GiB
 44  119653565850   91.9 GiB  429.0 GiB     222.9 GiB
 45  148902215280  114.4 GiB  543.4 GiB     277.4 GiB
 46  184509266760  141.8 GiB  685.2 GiB     343.7 GiB
 47  227692286640  174.9 GiB  860.1 GiB     424.1 GiB
 48  279871768995  215.0 GiB    1.0 TiB     521.3 GiB
=== ============= ========== ========== =============


Game model
==========

We will abstract away the game and suppose we have a description of game states
and two function ``successors :: State -> Iterator (States, Nat)`` and
``predecessors :: State -> Iterator States`` with the obvious semantics (the
``Nat`` in ``successors`` being the reward of a given play). Additionally, we
are going to restrict ourselves to games where the number of pieces on the
board increases and require a function ``iter_states :: Nat -> Iterator States``
iterating on every game state having the specified number of pieces on the
board.

Score optimal
-------------

Le score optimal d'une configuration est le maximum que l'on peut espérer
atteindre même si l'adversaire joue de manière parfaite. On a donc score(u) =
-n si u est terminale à n graines et score(u) = max(k - score(v) for (k, v) in
successors(u)) sinon.


Complexity model
================

TODO


Algorithme
==========

Présentation
------------

Pseudo-code:

.. code:: python

   def analysis():
       table[0] = 0
       for n in range(1, M+1):   # itération
           init_row(table, n)
           for i in range(n+1):  # étape
               sat = n - i
               for u in iter_states(n):
                   match table[u] with:
                       Stable(_): pass
                       Instable(x, s) if x == sat or s == 0:
                           table[u] = Stable(x)
                           for v in predecessors(u):
                               propagate(table, v, x, sat)
       return table

   def init_row(table, n):
       for u in iter_states(n):
           x, s = -n, 0
           for (v, k) in successors(u):
               s += 1
               if k > 0:
                   x = max(x, k - table[v])
           table[u] = Instable(x, s)

    def propagate(table, u, y, sat):
        match table[u] with:
            Stable(_): pass
            Instable(x, s) if x == sat or -y == sat or s == 0:
                x = max(x, -y)
                table[u] = Stable(x)
                for v in predecessors(u):
                    propagate(table, v, x, sat)
            Instable(x, s):
                x = max(x, -y)
                s -= 1


Complexité en temps
-------------------

Les opérations nous intéressant ici sont les accès à la table principale. On
essaye donc de compter leur nombre précisément. On peut cependant également
montrer que la complexitée est équivalente lorsque l'on compte également les
opérations classiques (instructions CPU).

On suppose qu'il existe K tel que #successors(u) <= K et
#predecessors(u) <= K (pour l'awalé, K=12). De plus on notera
f(n)=#iter_states(n) (pour l'awalé, f(n)=binom(11+n, 11)).


Coût de l'itération n
^^^^^^^^^^^^^^^^^^^^^

L'initialisation coûte f(n) + X où X est le nombre de coups donnant un gain non
nul pour les configurations à n graines. On majore X par K*f(n). Il s'agit
d'une borne assez large car on suppose que chaque configuration a K
successeurs, tous étant des coups à gain. Pour l'awalé, quelque chose de plus
raisonnable expérimentalement (voir src/bin/stats.rs) serait probablement f(n)
mais il est probablement impossible de montrer que c'est une majoration (sans
les compter une par une).

On remarque que le coût d'un appel à propagate est égal au nombre d'appels
récursifs déclenchés (plus 1 pour l'appel initial). Ce nombre est 0 si la
configuration n'est pas instable et dans les conditions de devenir stable. De
plus, après au plus K appels à propagate(.., u, ..), u devient stable. Comme on
appelle probablement beaucoup plus que K fois propagate sur chaque
configuration, on peut considérer que chaque appel coûte 1 et ajouter le coup
fixe K*f(n) au total de l'itération.

Pour la stabilisation, chacune des n+1 étapes (=boucle sur i) itère sur toutes
les configurations. L'opération effectuée coûte K+1 si la configuration est
dans la condition de devenir stable et 1 sinon. Ainsi sur pour une
configuration donnée, sur toutes les étapes de la stabilisation, au plus une
coute K+1 soit un coût de K + 1 + n.

Le coût total de l'itération en prenant en compte le cout de propagate suivant
l'argument donnée précédemment est donc (2*K + 1 + n)*f(n).

On peut affiner ce résultat de beaucoup en séparant les configurations à n
graines en l'ensemble de taille A (resp B) des configuration qui on été
"stabilisée" par propagate (resp une itération de stabilisation). On obtient
alors une complexité de::

            K*A + (n+1)*A + (K + 1)*B + n*B
             ^         ^        ^        ^
      sur-cout         |    sur-cout     |
      propagate        |    itération    |
                   itération          itération
                   simple             simple

Soit de manière simplifiée: (K + n + 1)*(A + B) = (K + n + 1)*f(n).

Au final on obtient le nombre d'accès mémoire: (K + n + 2)*f(n) + X

Quelques chiffres
^^^^^^^^^^^^^^^^^

On donne ici les chiffres de quelques instanciations concrêtes de l'équation
ci-dessus. On utilisera précisément la formule suivante pour le calcul du coût
d'une itération (on a donc choisi X=f(n)): cost(n) = (14+n)*binom(11+n, 11).

=== ==========
  n       cost
=== ==========
  0  1.400e+01
  1  1.800e+02
  2  1.248e+03
  3  6.188e+03
  4  2.457e+04
  5  8.299e+04
  6  2.475e+05
  7  6.683e+05
  8  1.663e+06
  9  3.863e+06
 10  8.465e+06
 11  1.764e+07
 12  3.515e+07
 13  6.740e+07
 14  1.248e+08
 15  2.241e+08
 16  3.911e+08
 17  6.657e+08
 18  1.107e+09
 19  1.803e+09
 20  2.879e+09
 21  4.516e+09
 22  6.967e+09
 23  1.059e+10
 24  1.585e+10
 25  2.343e+10
 26  3.420e+10
 27  4.934e+10
 28  7.039e+10
 29  9.941e+10
 30  1.390e+11
 31  1.926e+11
 32  2.646e+11
 33  3.605e+11
 34  4.872e+11
 35  6.537e+11
 36  8.709e+11
 37  1.152e+12
 38  1.515e+12
 39  1.980e+12
 40  2.572e+12
 41  3.322e+12
 42  4.269e+12
 43  5.456e+12
 44  6.940e+12
 45  8.785e+12
 46  1.107e+13
 47  1.389e+13
 48  1.735e+13
=== ==========

Correction
----------

Théorème de correction: pour toute configuration u, table[u] = score(u).

On peut prouver la correction de la construction de la table par récurrence sur
n. L'initialisation est triviale.


Lemme
^^^^^

On note #u le nombre de graines de la configuration u.

Soit u_0. Si score(u_0) = #u_0 - x alors il existe une suite u_1..k telle que:

- k >= 1
- u_{i+1} \in successors(u_i)       pour 0 <= i < k
- #u_i = #u_0                       pour 0 <= i < k
- #u_k < #u_0
- score(u_i) = (-1)^i * (#u_i - x)  pour 0 <= i <= k

Par la suite on ne s'intéressera qu'aux telles suites de longueur minimale et
on notera @u_0 (=k) leur longueur.

On peut se convaincre facilement de ce lemme en observant qu'on peut expliciter
une telle suite en prenant une *variation principale* (optimale) jusqu'au
prochain coup rapportant des points. En effet, dans la variation principale, on
a score(u_i) = #u_i - #u_{i+1} - score(u_{i+1}). De plus pour toute variation,
le nombre de graine reste le même jusqu'à ce qu'il décroisse. D'où le résultat
en remplaçant les score(u_0) par #u_0 - x.


Itération n
^^^^^^^^^^^
Soit n >= 1, on suppose que la table est bien construite pour  [0, n) et on
analyse l'itération n.

Pour i dans [-1,n], après l'étape i, abs(score(u)) >= n - i implique table[u] =
Stable(score(u)). On en déduit qu'après l'étape n toutes les configurations
sont stables.

Démonstration par récurrence sur i.

- Initialisation: si #u = n alors score(u) \in [-n, n], la propriété est donc
  vraie pour i=-1 (avant le début de la boucle).

- Hérédité: On suppose la propriété vraie pour i-1 et on montre qu'au cours de
  l'itération i, toutes les configurations u telles que abs(score(u)) = n - i
  sont stabilisées.

  On procède par récurrence sur @u.

  - Initialisation:

    - Soit u telle que @u = 1 et score(u) = n-i. D'après le lemme il existe u_1
      telle que score(u_1) = #u_1-i et #u_1 < #u. Par hypothèse de récurrence
      sur n, u_1 est déjà stable, donc son score a été propagé à u dans la
      procédure d'initialisation. Soit u est déjà stable auquel cas c'est fini,
      soit u = Instable(n-i, _) et alors comme ``sat`` = n - i, u va être
      stabilisé à son score.

    - Soit u telle que @u = 1 et score(u) = i-n. Soit v un successeur de u, par
      définition, score(v) >= n-i. Si #v = n alors par le paragraphe précédent,
      v est stable et a donc été propagé à u. Si #v < n, v est également
      stable.  Ainsi tout les successeurs de u sont stable, donc u est stable.

  - Hérédité: soit k \in [1,..] on suppose que pour tout u telle que @u <= k et
    abs(score(u)) = n-i, u est stable.

    - Soit u telle que @u = k+1 et score(u) = n-i. D'après le lemme il existe
      u_1 telle que @u_1 = k et score(u_1) = i-n. Par hypothèse de récurrence,
      u_1 est stable, elle a donc propagé son score à u qui a été stabilisé par
      saturation.

    - Soit u telle que @u = k+1 et score(u) = i-n. Soit v un successeur de u,
      par définition, score(v) >= n-i. Si #v = n alors d'après le paragraphe
      précédent v est stable et de même si #v < n. Ainsi u est stable car tous
      ses successeurs le sont.

cqfd.
