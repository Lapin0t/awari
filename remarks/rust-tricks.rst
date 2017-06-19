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
s'agit donc d'une forme (simplifiée) de type dépendant (type :math:`\Pi`).
Cependant cette construction n'est pas *first-class* en Rust: le polymorphisme
sur les entiers n'est pas possible. Plusieurs issues discutent ceci (`integer
generics <intgenerics_>`_, `:math:``\Pi`-types <>`)
