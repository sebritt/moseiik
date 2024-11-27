# Projet Qualité Logiciel

Moseiik est un projet qui permet de générer des mosaïques d'images à partir d'un corpus de vignettes (appelées tiles) et d'une image de référence (appelée target).

Le code source se trouve dans le fichier `src/main.rs` et peut être exécuté en lançant depuis le dossier `moseiik` la commande :

```bash 
cargo run --release -- --image "assets/target-small.png" --tiles "assets/tiles-small"
``` 

le `--` permettent de faire la différence entre les paramètres de cargo et ceus de notre application. 

Cette commande recompose l'image `assets/target-small.png` en utilisant les vignettes du dossier `assets/tiles-small`. Le résultat est ensuite sauvegardé dans l'image `out.png`. Si le code fonctionne bien, les deux images `target-small` et `out` devraient être les mêmes, puisque les vignettes dans `assets/tiles-small` sont des sous-parties de l'image `target-small` et permettent donc de reconstruire parfaitement l'image. Pour avoir la liste des options, vous pouvez lancer la commande `cargo run -- -h`. 

## Description du projet

Dans ce projet, les images sont gérées par le [crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) [image](https://docs.rs/image/0.24.8/image/). Toutes les images sont de type [RgbImage](https://docs.rs/image/0.24.8/image/type.RgbImage.html) qui implemente le trait [EQ](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html), ce qui signifie que l'égalité entre deux images peut être testée en utilisant le double égal `==`.

Différentes étapes sont nécessaires pour créer la mosaïque :

### Préparation des vignettes

La préparation des vignettes se fait via la fonction `prepare_tiles`. Elle liste l'ensemble des images du dossier spécifié par le paramètre `--tiles`. Ces images sont ensuite chargées en mémoire et redimensionnées à la taille spécifiée par le paramètre `--tile-size`. Cette fonction renvoie un type `Result<Vec<RgbImage>, Box<dyn Error>>`. Le type [Result](https://doc.rust-lang.org/std/result/) (voir aussi dans le [livre rust](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=result#recoverable-errors-with-result)) permet de traiter les erreurs avec un match. S'il y a une erreur, elle est gérée, s'il n'y en a pas, on peut alors récupérer le `Vec<RgbImage>` contenant les vignettes redimensionnées.

### Préparation de la référence

Le paramètre `--scaling` permet de redimensionner la taille de la référence. Celle-ci est ensuite rognée de façon à ce que la taille de la référence soit un multiple de celle des vignettes (pour éviter d'avoir à gérer les effets de bords). 

Par exemple, si l'image d'entrée est de taille 1920 x 1080 et que l'on applique un scaling x2 et que l'on utilise des vignettes de taille 25 x 25, alors l'image de sortie sera de taille :

```
(1920 x 2 - (1920 x 2) mod 25, 1080 x 2 - (1080 x 2) mod 25) = (3825, 2150)
```

### Recherche des vignettes optimales

À la suite des deux traitements précédents, la taille de la référence est un multiple de la taille des vignettes : (w x tile-size, h x tile-size). Ainsi, dans la fonction `compute_mosaic`, on boucle avec un pas de tile-size sur les lignes (de 0 à w) et les colonnes (de 0 à h). Pour chaque bloc de l'image référence, on cherche la vignette la plus proche en utilisant la fonction `find_best_tile` qui elle-même appelle la fonction `l1`.

Le calcul sur les lignes peut être parallélisé en utilisant le paramètre `--num-thread $N`.

Le calcul de la distance L1 peut être effectué avec du SIMD en utilisant le paramètre `--simd`. Si ce paramètre est donné, la fonction `get_optimal_l1` permet de sélectionner la fonction la plus adaptée en fonction de l'architecture disponible. Si le code est exécuté sur une architecture ARM, la fonction `l1_neon` est appelée. Si l'architecture est x86, alors la fonction `l1_x86_sse2` est appelée. Si le paramètre `--simd` n'est pas fourni, la fonction `l1_generic` est appelée.

Le code comporte des [attributs](https://doc.rust-lang.org/beta/core/arch/index.html) qui permettent de ne compiler que les fonctions qui sont supportées par l'architecture. Par exemple, sur un ordinateur en x86, les fonctions (y compris les fonctions de test) précédées `#[cfg(target_arch = "aarch64")]` ne seront pas compilées. 

## Implémentation des tests

Pour les tests, vous pouvez créer des données vous-même, ou bien utiliser les images du dossier `assets` (un exemple de fonction permettant d'ouvrir une image est disponible au début de la fonction `prepare_target`).

Une base d'images pouvant servir de vignettes est également disponible au lien https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip.  

Les tests peuvent être executés en utilisant la commande :

```bash
cargo test --release
```

L'option `--release` permet de compiler en mode release (et non debug), ce qui augmente significativement la rapidité d'execution du code.

### Tests unitaires

Les tests unitaires permettent de tester les fonctions intermédiaires avec des cas simples pour vérifier qu'elles font ce qui est attendu. 

Les fonctions à tester sont `l1_neon`, `l1_x86_sse2`, `l1_generic`, `prepare_target` et `prepare_tiles`.

Ces tests sont à inclure à la fin du fichier `src/main.rs` (un squelette est déjà disponible et doit être complété). Ils doivent tester les différentes versions de la distance L1 (`l1_neon`, `l1_x86_sse2` et `l1_generic`). La fonction `prepare_tiles` doit retourner des tiles de la bonne taille (pas besoin de tester le contenu des images, seulement leurs tailles). Finalement, la fonction `prepare_target` doit également retourner une image avec une taille cohérente. 

### Tests d'intégration

Les tests d'intégration se trouvent dans le dossier `tests`. De même que pour les tests unitaires, un squelette est disponible. Ces tests doivent tester la fonction `compute_mosaic` pour les différentes architectures processeurs prises en compte.

Le dossier assets contient une image nommée `assets/ground-truth-kit.png` qui est un exemple de mosaïque obtenue avec la base téléchargeable et l'image `assets/kit.png`. Les vignettes sont de taille 25 et l'image est conservée à sa taille d'origine. Il pourrait être intéressant de regénérer cette image et vérifier que l'image générée est identique à la vérité terrain.

Attention, les tests d'intégration ne sont pas éxecutés si au moins un test unitaire ne passe pas.

## Executer les tests localement avec Docker

### Création d'une image Docker

L'image docker permet d'exécuter le code et doit donc au minimum contenir le code source ainsi que `cargo`.
Il est également possible d'y télécharger les vignettes de la base en ligne.

Il est possible avec Docker de passer des paramètres à une commande exécutée dans l'image Docker. Pour cela, il suffit d'utiliser un entrypoint en terminant le Dockerfile avec la ligne :

```Dockerfile
ENTRYPOINT [ "cargo", "test", "--release", "--" ]
```

L'objectif est qu'une fois le conteneur créé, il soit possible de lancer l'application Moseiik simplement sur x86 et arm sans avoir à regénérer l'image Docker.

Attention, pour que l'image soit utilisable avec les deux architectures il faut s'assurer que l'image de base l'est également. Les compatibilité des images sont indiqués dans docker hub.

### Execution des tests avec Docker

Une fois le Dockerfile créé, vous pouvez le compiler et exécuter les tests. Votre ordinateur étant probablement sous architecture x86, il ne sera pas capable d'exécuter les tests ARM (ou inversement si vous utilisez par exemple un MacBook récent ou une Raspberry Pi). Une possibilité est d'installer Qemu pour émuler l'architecture non supportée par votre machine. 

L'installation de Qemu pour tester toute les architectures est optionnelle, mais cela peut permettre d'identifier des bugs dans les tests avant de passer à l'intégration dans GitHub. Docker Desktop est capable de d'utiliser cette couche d'émulation de manière transparente.

Pour executer les tests sur une architecture spécifique, les options `-t` et `--platform` de la commande [docker build](https://docs.docker.com/engine/reference/commandline/build/) peuvent vous être utiles. `-t` permet de donner un nom à votre image, nom qui sera nécessaire pour lancer l'image avec la commande [docker run](https://docs.docker.com/engine/reference/commandline/run/). L'option `--platform` (également disponible pour [docker run](https://docs.docker.com/engine/reference/commandline/run/)) vous permet de spécifier l'architecture cible si celle-ci est supportée par votre machine ou que Qemu est installé. 

## Utilisation de GitHub Actions pour executer les tests

L'intégration continue va être faite avec Github Actions. L'objectif final est la chaine de CI capable de récupérer les sources en Rust de Moseiik, exécuter les tests unitaires ainsi que les tests d'intégrations, le tout à la fois sur une architecture x86_64 (ou amd64) et sur une architecture arm64 (ou aarch64).

Une fois que le Dockerfile à été testé et validé pour x86 et arm, mettez en place la CI.

Idéalement, le fichier `yaml` ne doit être composé que d'un seul `job` pour exécuter la CI sur les 2 architectures à la fois en utilisant une [matrix](https://docs.github.com/en/actions/using-jobs/using-a-matrix-for-your-jobs).

Cette CI va être composée de 4 étapes majeures :
1. Mise en place conditionnel de QEMU (manuellement ou avec une [action Github](https://github.com/marketplace/actions/docker-setup-qemu)).
2. Récupération du `Dockerfile`.
3. Génération de l'image Docker.
4. Exécution de l'image Docker pour lancer les tests.

Le déroulement de ce workflow Github Actions doit normalement être très similaire à l'exécution des tests dans un conteneur local.
