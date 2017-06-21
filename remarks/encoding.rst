Encodage compact de plateaux d'Awalé
====================================

Le but ici est de construire une fonction de *ranking* pour les plateaux
d'awalé, c'est-à-dire une fonction qui donnera leur indice entier tel qu'un
ordre choisi (ici colex) coincide avec l'ordre usuel sur les indices. Cette
fonction de ranking est également une fonction de hachage parfaite, qui possède
la propriété d'être "compacte", c'est-à-dire qu'elle est à valeur dans
:math:`[0,N)` où :math:`N` est le nombre d'éléments de l'ensemble de départ.

On s'intéresse aux plateaux d'Awalé à :math:`P` puits (:math:`P = 2Q`).

On remarque que l'on peut transformer (bijectivement) le vecteur :math:`A` d'un
plateau d'awalé en un vecteur :math:`B` où:

.. math::

   B_0 = A_0
   \forall k < P, B_{k+1} = A_{k+1} + B_{k} + 1


.. note::

   Cette transformation ressemble à la construction en théorie des ensemble des
   suite finie.


On code alors le vecteur :math:`B` par le nombre qu'il représente, vu dans le
système numéral combinatoire (knuth_):

.. math::

   \text{enc}(B) = \Sum_{i=0}^{P-1} \binom{B_i}{i+1}


Algorithmes
-----------

Algorithmes exprimés en python:

.. code:: python

    def encode(xs):
        g, c = 0, 0
        for i, x in enumerate(xs):
            c += x
            g += binom(i + 1, c + i)
        return g

    def decode(g):
        xs = [0] * n_pits
        for i in reversed(range(P)):
            n = binom_inv(i+1, g)
            g -= binom(i+1, n)
            xs[i] = n
        for i in reversed(range(1, P)):
            xs[i] = xs[i] - xs[i-1] - 1
        return xs


On aura préalablement défini les fonctions ``binom(k,n)`` et ``binom_inv(k,b)``
calculant respectivement le coefficient binomial :math:`\binom{n}{k}` et le
plus grand :math:`n` tel que :math:`\binom{n}{k} \leq b` (on peut précalculer
la table de manière à avoir la première en O(1) et la seconde en O(log n) ie
O(1) pour P fixé).

On a donc directement des complexités O(P) pour les deux fonctions.


Bijectivité de :math:`enc`
--------------------------

On montre que si on énumère les vecteurs dans l'ordre lexicographique (de
droite à gauche), blabla, dénombrement TODO. Donc l'encodage est bien injectif
et surjectif.

Montrons la correction de l'algorithme de décodage, on montre:

.. math::

   \Sum_{i=0}^{t-1} \binom{c_i}{i+1} < \binom{c_{t-1} + 1}{t}

Par récurrence sur :math:`t`, on a bien pour :math:`c_0 < c_0 + 1` pour
:math:`t = 1` puis on suppose la propriété vraie pour :math:`t`. On a
:math:`c_{t-1} + 1 \leq c_t` donc :math:`\Sum_{i=0}^{t-1} \binom{c_i}{i+1} \leq
\binom{c_t}{t}`. Or on a :math:`\binom{c_t + 1}{t+1} = \binom{c_t}{t} +
\binom{c_t}{t+1}` donc en rajoutant :math:`\binom{c_t}{t+1}` des deux côtés de
l'équation on obtient bien la propriété pour :math:`t+1`.

De plus on a directement :math:`\binom{c_{t-1}}{t} \leq \Sum_{i=0}^{t-1}
\binom{c_i}{i+1}`. On en déduit donc que :math:`c_{t-1}` est bien le plus grand
entier :math:`a` tel que
:math:`\binom{a}{t} \leq \text{enc}(c) - \Sum_{i=t}^{P-1} \binom{c_i}{i+1}`,
d'où la correction de l'algorithme de décodage.


Itération sur les positions
---------------------------

On remarque facilement que la position à :math:`n` graines qui a le plus petit
code est :math:`(0, 0, \dots, n)`, son code est :math:`\binom{P}{P+n-1}`. On a
vu que les positions étaient triées par nombre de graînes croissant pour
l'ordre induit par les codes (:math:`c_{P-1} = n + P` avec :math:`n` le nombre
de graînes). Ainsi on en déduit une manière simple d'itérer sur toutes les
positions à :math:`n` graînes:

 .. code:: python

    map(decode, range(binom(P, P+n-1), binom(P, P+n)))


Une méthode plus efficace
^^^^^^^^^^^^^^^^^^^^^^^^^

Cette approche fait partie de la catégorie ranking-unranking: elle est simple à
comprendre mais a le défaut de faire beaucoup d'appels à ``decode`` qui sont
coûteux du fait de l'appel à ``binom_inv``. On peut l'affiner en générant
directement des vecteurs sans passer par les codes.

Cependant générer directement des vecteurs sommant à une constante n'est pas
trivial: pour s'en rendre compte on peut par exemple essayer de tirer
uniformément un tel vecteur au moyen d'un générateur pseudo-aléatoire d'entier.
Le représentation sous forme de vecteur croissant (les :math:`c_k`) est
particulièrement pratique: la somme fixe se traduit par un :math:`c_{t-1}`
fixé, et la propriété d'être croissant s'obtient facilement en triant en
vecteur quelconque.

On peut déduire des paragraphes précédants qu'il y a :math:`\binom{P+n-1}{P-1}`
plateaux à :math:`n` graines, mais on peut aussi voir cela du point de vue du
vecteur de coefficients: choisir un vecteur croissant de taille :math:`P` tel
que :math:`c_{t-1} = n + P - 1` revient à choisir :math:`P-1` nombres dans
l'intervalle :math:`[0,P+n-1)`. On peut représenter un tel choix par un vecteur
de bits de taille :math:`P+n-1` contenant exactement :math:`P-1` ``1``.

Comme tout n'arrive pas par hasard, il existe une manière d'itérer sur tous les
tels vecteurs de bits (en le voyant comme des entiers machine) exactement dans
le même ordre lexicographique (ref: bithacks_) (note: ``ctz`` signifie "count
trailing zeros", c'est souvent une primitive machine):

.. code:: python

   def next_bv(x):
       t = x | (x - 1)
       return (t + 1) | (((~t & (t + 1)) - 1) >> (ctz(x) + 1))

   def iter_bv(k, n):
       x = (1 << k) - 1
       last = x << n - k
       while x <= last:
           yield x
           x = next_bv(x)


De ce vecteur de bits, on pourrait passer à une représentation sous forme de
vecteur de coefficients puis enfin de vecteur de somme fixée, mais on peut
facilement écrire une fonction efficace passant du vecteur de bits au vecteur
de somme fixée:

.. code:: python

   def bv2list(x, n):
       xs = [0] * P
       x = x | (1 << P + n - 1)  # on rajoute le dernier bit qui est implicite
       for i in range(P):
           a = ctz(x)
           x >>= a + 1
           xs[i] = a
       return xs


.. _knuth: Bitwise tricks and techniques, 2005, TAoCP 4, Fascicle 1.
.. _bithacks: http://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation
