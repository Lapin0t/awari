Encodage compact de plateaux d'Awalé
====================================


On s'intéresse aux plateaux d'Awalé à :math:`2P` puits.

On remarque que l'on peut transformer (bijectivement) le vecteur :math:`A` d'un
plateau d'awalé en un vecteur :math:`B` où:

.. math::

   B_0 = A_0
   \forall k < P, B_{k+1} = A_{k+1} + B_{k} + 1


.. note:: Cette transformation ressemble à l'encodage en théorie des ensemble
   de la suite finie :math:`A` en l'ensemble :math:`B`.


On code alors le vecteur :math:`B` par:

.. math::

   \text{enc}(B) = \Sum_{i=0}^{P-1} \binom{B_i}{i+1}


Bijectivité de :math:`enc`
--------------------------

On montre que si on énumère les vecteurs dans l'ordre lexicographique (de
droite à gauche), blabla, dénombrement TODO.

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
