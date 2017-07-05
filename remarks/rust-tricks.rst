Quelques tricks rencontrés en Rust
==================================


Initialisation de variables globales statiques
----------------------------------------------

Rust supporte très mal l'exécution de code à la compilation (uniquement des
expressions sans flow-control, cf les ``const fn``, la rfc_ et l'issue_
associée). Cependant il est parfois préférable d'initialiser les variables
globales au début de l'exécution (surtout s'il s'agit d'un gros tableau qui
prendrait de la place inutilement dans le binaire). On se heurte alors au
problème d'initialiser cette variable globale statique (et immutable): pour
modifier une variable globale on est obligé d'utiliser du code *unsafe*. Une
bibliothèque Rust -- lazy_static_ -- fourni une macro permettant exactement de
faire ceci (au détail près qu'il ne gère que les reférences). Elle tire parti
de l'overloading du déréférencement par le trait ``Deref`` pour initializer la
référence lors de son premier déréférencement.

.. code:: rust

   lazy_static! {
       static ref TABLE : [u32; 1337] = mk_table();  // executed at runtime
   }


.. _rfc: https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md
.. _issue: https://github.com/rust-lang/rust/issues/24111
.. _lazy_static: http://rust-lang-nursery.github.io/lazy-static.rs/lazy_static/index.html


Du polymorphisme sur les entiers (du pauvre)
--------------------------------------------

Les tableaux Rust contiennent leur longueur dans leur type: ``[T; n]``, il
s'agit donc d'une forme (simplifiée) de types-Pi (dépendant d'entiers
constants). Cependant cette construction n'est pas *first-class* en Rust: le
polymorphisme sur les entiers n'est pas possible. Plusieurs `RFC <pi_types_>_`
discutent de ceci et proposent notamment d'introduire d'abord les types
dépendants sur les constantes (un introduisant des *kinds* et en ajoutant
``Const :: * -> Kind``).

En pratique il y a quelques solutions pour contourner ce manque aujourd'hui de
ce que le C++ appelle tout simplement les templates d'entiers:

- la bibliothèque typenum_ qui construit des entiers au type-level, c'est
  plutôt lourd et pas intégré au langage, mais c'est la solution la plus
  théoriquement propre
- la feature *associated_consts*, des *Traits* qui permet d'écrire un trait et
  de l'implémenter pour différente tailles, on peut éviter la duplication du
  code en donnant une implémentation par défaut pour la plupart des méthodes,
  il suffit donc de faire des blocs ``impl`` quasiment vides pour toutes les
  tailles que l'on veut
- la bonne vieille constante globale définie par un flag de compilation (comme
  un ``DEFINE`` en C)

La première méthode m'a semblée trop lourde (et prenant le parti de la théorie
pour mieux s'asseoir sur l'utilisation pratique), j'ai tenté la deuxième
pendant un moment avant de m'arracher les cheveux sur des features instables
pour de `bonnes raisons <assoc_consts>`_. La dernière méthode est donc la plus
efficace du point de vue de la fiabilité et de la praticité, c'est celle qui
est actuellement utilisée dans le code.


.. _pi_types: https://github.com/rust-lang/rfcs/issues/1930
.. _typenum: http://paholg.com/typenum/typenum/index.html
.. _assoc_consts: https://github.com/rust-lang/rust/issues/34344
