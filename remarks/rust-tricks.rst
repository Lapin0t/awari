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


Une couche d'abstraction souple pour du stockage
------------------------------------------------

Assez vite s'est posé le problème de pouvoir coder plusieurs versions du
stockage de la table: dans la RAM, sur disque puis de manière hybride: sur
disque avec un cache en RAM. Comme c'est propice à erreurs de continuellement
modifier le code de l'analyse rétrograde, j'ai codé une couche d'abstraction
qui peut être instanciée avec différentes backend. Le problème a été de trouver
une abstraction qui colle bien aux 2 modèles.

L'api standard pour un tableau en RAM::

   trait RAM<T> {
       fn borrow(&Self, usize) -> &T;
       fn borrow_mut(&mut Self, usize) -> &mut T;
   }

L'api standard sur disque::

   trait Disk<T> {
       fn read(&mut Self, usize) -> T;
       fn write(&mut Self, usize, T);
   }

Elles ne se ressemblent pas! Dans la partie RAM il suffit de renvoyer un
pointeur, mutable ou pas vers le stockage principal et dès que le programmeur
utilise ce pointeur les données seront modifiées alors que pour la version
disque la version dans la RAM n'est qu'une copie de travail qu'il faudra
ré-écrire sur le disque à la fin.

On va donc utiliser 3 traits Rust bien pratiques:

``Deref``
   Ce trait a pour définition::

      trait Deref {
          type Target;
          fn deref(&self) -> &Target;
      }

   Il permet d'overloader l'opérateur de déréférencement ``*``. Cela permet par
   exemple de créer des types qui s'utilisent comme des pointeurs mais peuvent
   implémenter de la logique supplémentaire. Par exemple ``Vec<T>`` implémente
   ``Deref<Target=[T]>``.

``DerefMut``
   Il s'agit d'un sous-trait de ``Deref`` qui fait la même chose mais pour des
   pointeurs mutables: ``deref_mut(&mut self) -> &mut Target;``

``Drop``
   Ce trait permet de customizer le finaliseur d'un objet: il contient une
   simple fonction ``drop(&mut self);`` qui s'exécutera juste après que l'objet
   devienne innacessible.

On peut donc progresser avec::

   trait Storage<T> {
       type Proxy: Deref<Target=T>;
       type ProxyMut: DerefMut<Target=T>;
       fn index(&self, usize) -> Proxy;
       fn index_mut(&mut self, usize) -> ProxyMut;
   }

De cette manière, l'implémentation du trait pour le stockage en RAM est trivial
et pour le modèle sur disque, on peut faire quelquechose de cet ordre (je
montre juste la partie non-mutable, l'autre est semblable)::

   struct Disk<T> { fd: File }
   struct Proxy<T> { owner: Disk<T>, data: T, index: usize }

   impl<T> Deref for Proxy<T> {
       type Target = T;
       fn deref(&self) -> &T {
           &self.data
       }
   }

   impl<T> Drop for Proxy<T> {
       fn drop(&mut self) {
           self.owner.write(self.index, self.data);
       }
   }

   impl<T> Storage<T> for Disk<T> {
       type Proxy = Proxy<T>;
       fn index(&self, i: usize) -> Self::Proxy {
           Proxy { owner: self, data: self.read(i), index: i }
       }
   }

Seulement un problème apparait: on ne veut pas prendre l'ownership de l'objet
``Disk<T>`` avec le champ ``owner``, on voudrait simplement un borrow ``owner:
&'a Disk<T>``. Seulement pour cela il faut introduire un paramètre de durée de
vie (lifetime): ``Proxy<'a, T>`` et on voudrait pouvoir écrire::

   type Proxy<'a> = Proxy<'a, T>;
   fn index<'a>(&'a self, usize) -> Self::Proxy<'a>;

Seulement Rust ne supporte par encore les types associés d'ordre supérieur,
donc il faut ruser. J'ai donc rajouté une séparation pour sortir les types
``Proxy`` et ``ProxyMut`` du trait, qui a juste a fournir les bonnes fonctions,
ça donne::

   trait Backend<T> {
       type Handle;
       fn get_handle(&Self, usize) -> Self::Handle;
       fn deref_handle<'a>(&'a Self, &'a Self::Handle) -> &'a T;
       fn deref_handle_mut<'a>(&'a mut Self, &'a mut Self::Handle) -> &'a mut T;
       fn cleanup(&mut Self, &Self::Handle);
   }

Pour un peu plus de détails sur la version finale voir le fichier
``src/storage.rs``.
