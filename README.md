# Introduction of Design Pattern with Rust

# Disclaimer

Ce projet est une traduction de cet article : [article](https://blog.devgenius.io/design-patterns-with-rust-8bfea60f6447)



# Introduction

Les modèles de conception sont des solutions réutilisables à des problèmes courants dans la conception de logiciels. Ils fournissent des modèles sur la manière de résoudre les problèmes de conception récurrents et les défis architecturaux.

Les modèles de conception rendent le code plus flexible, plus réutilisable et plus facile à maintenir. Ils vous aident à construire des applications robustes, évolutives et faciles à maintenir.

Dans cet article, nous allons explorer plusieurs modèles de conception importants utilisés dans la programmation Rust. Nous verrons des exemples de mise en œuvre de chaque modèle et nous examinerons quelques cas d'utilisation pratiques.

# Qu'est-ce qu'un modèle de conception ?

Les modèles de conception fournissent des moyens normalisés de résoudre des problèmes courants de conception orientée objet. Ils reprennent des solutions qui ont évolué au fil du temps et dont l'efficacité a été prouvée.

Les principaux avantages de l'utilisation des modèles de conception sont les suivants :

* éviter de réinventer la roue
* Favoriser la réutilisation
* Rendre le code plus robuste et plus facile à maintenir
* Fournir un vocabulaire de conception commun

Les modèles de conception ne dépendent pas du langage de programmation et peuvent être mis en œuvre dans n'importe quel langage orienté objet. Cependant, différents langages peuvent implémenter le même modèle différemment en fonction de leurs capacités et de leurs caractéristiques.

# Pourquoi les modèles de conception sont-ils utiles ?

Les modèles de conception permettent de résoudre de nombreux problèmes récurrents dans la conception de logiciels orientés objet. L'utilisation de modèles de conception éprouvés présente les avantages suivants :

* Développement rapide : Ne perdez pas de temps sur des problèmes déjà résolus. Réutilisez les solutions éprouvées.
* Robustesse : Les modèles de conception sont des solutions éprouvées qui ont fonctionné dans le passé.
* Maintenabilité : Rend la conception plus flexible et plus facile à comprendre pour les développeurs.
* Productivité : Les développeurs n'ont pas besoin de réinventer des solutions à des problèmes courants.
* Diminution du coût du changement : Si les exigences changent à l'avenir, les modèles de conception rendent la conception plus souple et plus facile à modifier.

En bref, les modèles de conception accélèrent le processus de développement en fournissant des solutions testées et réutilisables aux problèmes courants.


# Modèles de conception en Rust

    Rust est un langage de programmation moderne axé sur la sécurité, la vitesse et la concurrence. Il s'agit d'un langage de compilation doté de puissantes vérifications statiques.

Voici quelques-unes des principales caractéristiques de Rust :

* Abstractions à coût nul : Les abstractions en Rust n'entraînent pas de surcoût à l'exécution.
* Sécurité de la mémoire : Grâce à son système de propriété, le compilateur Rust garantit la sécurité de la mémoire et empêche les courses de données au moment de la compilation.
* Liaisons efficaces avec le langage C : Rust dispose d'une interface de fonctions étrangères qui permet d'appeler du code C.
* Sécurité de type : Rust est un langage statiquement typé, mais il dispose également d'une inférence de type.
* Concurrence sans course aux données : Rust assure la sécurité des threads grâce à son système de types et à ses règles de propriété.

Rust supporte plusieurs caractéristiques orientées objet telles que

* Les structures : Types de données agrégées similaires aux structs du langage C.
* Traits : Utilisés pour définir des interfaces/types abstraits. Similaire aux interfaces dans d'autres langages.
* Enums : Utilisées pour représenter les variantes/cas.
* Génériques : Utilisés pour écrire du code flexible et réutilisable.

Ainsi, bien que Rust ne soit pas un langage orienté objet traditionnel, il possède suffisamment de caractéristiques orientées objet pour implémenter la plupart des patrons de conception traditionnels. Dans la suite de cet article, nous allons explorer comment appliquer les patrons de conception en Rust.

# Modèles de conception créative

Les modèles de création permettent d'instancier des objets de manière flexible et réutilisable. Quelques exemples de modèles de création sont : le modèle du constructeur, le modèle de la méthode d'usine et le modèle du singleton. Explorons ces modèles en Rust.
Le modèle du constructeur

En raison de la structure de Rust et de son caractère immuable par défaut, ce pattern n'est pas applicable dans la plupart des cas. Cependant, si nous prenons le pattern tel quel et essayons de l'implémenter ici.

# Builder pattern

Le pattern builder sépare la construction d'un objet complexe de sa représentation. Pour ce faire, il utilise un objet constructeur qui reçoit chaque paramètre d'initialisation étape par étape et construit ensuite l'objet final. Cela permet de produire différents types et représentations d'un objet en utilisant le même code de construction.

En Rust, nous pouvons implémenter le modèle du constructeur comme suit :

    voir builder_pattern/

Ce processus de construction étape par étape nous permet de produire différentes représentations de l'objet Personne en réutilisant le même code.

# Modèles de conception structurelle

Les modèles de conception structurelle traitent de la composition des classes ou des objets. Ils garantissent qu'une classe utilise une autre classe, hérite d'une autre classe ou contient des instances d'autres classes d'une manière particulière.

## A. Modèle d'adaptateur

Le modèle de l'adaptateur convertit l'interface d'une classe en une autre interface attendue par le client. Il permet à deux interfaces incompatibles de fonctionner ensemble.

    voir adater_pattern/

Le DogAdapter adapte le chien à l'interface Lion en implémentant la méthode roar(). Le code client utilise l'interface Lion et n'est pas au courant de l'adaptation.

La création d'adaptateurs pour des architectures d'unités centrales incompatibles est un exemple de modèle d'adaptateur. L'adaptateur convertit le jeu d'instructions CPU  en instructions pour une autre architecture CPU.

## B. Modèle de décorateur

Le modèle du décorateur permet d'attacher dynamiquement des comportements supplémentaires à un objet en les plaçant dans des objets enveloppants. Il constitue une alternative flexible à la sous-classe pour étendre les fonctionnalités.

Voici un exemple de décorateur utilisé pour construire des pizzas :

    voir decorator_pattern/

Nous disposons d'un trait Pizza de base et d'une structure SimplePizza concrète. La structure ToppingDecorator enveloppe une Pizza et ajoute un coût de garniture. Elle implémente le trait Pizza, en déléguant à la pizza intérieure et en ajoutant le coût de la garniture. Cela nous permet d'ajouter dynamiquement des garnitures (décorateurs) à une pizza.

## C. Modèle de proxy

Le modèle proxy fournit un substitut ou une place pour un autre objet afin d'en contrôler l'accès. Un proxy agit comme une interface vers quelque chose d'autre et ajoute des fonctionnalités tout en appelant des méthodes sur l'objet original.

Voici un exemple d'utilisation du modèle proxy pour la mise en place de la mise en cache :

    voir proxy_pattern

La ProxyImage agit comme un proxy pour la RealImage. Lorsque sa méthode `show()` est appelée, elle vérifie si l'image réelle a déjà été mise en cache. Si c'est le cas, elle affiche simplement "Loading cached", sinon elle délègue à la RealImage et met le résultat en cache. Cette implémentation utilise le modèle de proxy pour ajouter une fonctionnalité de mise en cache à un chargeur d'images.

# Modèles de conception comportementale

Les modèles de conception comportementale concernent la communication entre les objets et la manière dont leurs responsabilités sont réparties. Examinons deux modèles comportementaux importants en Rust :

# Modèle d'observateur

Le modèle de l'observateur définit une dépendance d'un à plusieurs entre les objets de sorte que lorsqu'un objet change d'état, tous ses dépendants sont notifiés et mis à jour automatiquement.

En Rust, nous pouvons implémenter le modèle de l'observateur en utilisant des traits. Nous définissons un traitSubject pour l'objet observé et un trait Observer pour les observateurs :

Ceci démontre un modèle d'observateur de base où le SubjectImpl notifie tous les observateurs attachés lorsque son état change.

# Modèle de visiteur

Le modèle du visiteur sépare un algorithme de la structure de l'objet sur lequel il opère. Une application pratique du modèle du visiteur est la traversée d'un AST (Abstract Syntax Tree).

Nous définirons un trait Node et un enum Expression pour les différents nœuds de l'AST :

    voir visitor_pattern

Ceci démontre le modèle de visiteur en action ! Le visiteur Evaluator parcourt l'AST et évalue le résultat de l'expression finale.

Je poursuivrai l'implémentation d'autres modèles dans un autre article, car cet article devient trop long :)

# Conclusion

Les patrons de conception sont des solutions éprouvées à des problèmes de conception courants qui peuvent rendre notre code réutilisable, flexible et maintenable. Dans cet article, nous avons exploré quelques patrons de conception populaires et leur implémentation en Rust.

Le modèle de propriété et le système de types de Rust nous permettent d'implémenter les patrons de conception d'une manière très concise et sûre. Le modèle du constructeur nous permet de construire des objets complexes étape par étape d'une manière lisible. Le modèle des méthodes d'usine nous donne la possibilité de choisir différentes implémentations d'un objet en fonction de la situation. Le modèle du singleton garantit qu'un seul objet existe à la fois.

Le modèle de l'adaptateur permet à des interfaces incompatibles de fonctionner ensemble en adaptant une interface à une autre. Le modèle du décorateur ajoute dynamiquement un comportement aux objets en les enveloppant. Le modèle de proxy contrôle l'accès à un objet en utilisant un proxy à sa place.

Le modèle de l'observateur établit une dépendance d'un à plusieurs entre les objets, de sorte que lorsqu'un objet change d'état, tous ses dépendants en sont informés. Le modèle du visiteur nous permet d'étendre la fonctionnalité d'une hiérarchie d'objets sans modifier leurs interfaces en utilisant un objet visiteur.

En résumé, les modèles de conception nous fournissent des solutions qui rendent notre code plus flexible, plus réutilisable et plus facile à maintenir. En comprenant et en mettant en œuvre ces modèles, nous pouvons porter nos compétences en matière de rouille à un niveau supérieur et construire des systèmes logiciels robustes et évolutifs.

