# books

Service web auto-hébergé de bibliothèque personnelle : il garde une trace des livres lus et à lire, avec une note et un avis pour chacun. Les informations d'un livre peuvent être importées depuis [Open Library](https://openlibrary.org) ou saisies à la main.

> **État du projet :** en cours de développement, pas encore utilisable.

## Environnement de développement

Le développement se fait dans un conteneur [distrobox](https://distrobox.it) défini dans `distrobox.ini`. Son dossier personnel est isolé dans `.home/`, à la racine du projet, et n'est pas versionné.

### Prérequis

- [Podman](https://podman.io) (ou Docker)
- distrobox

### Créer l'environnement

1. Adapter le chemin `home=` de `distrobox.ini` à l'emplacement du projet sur la machine.
2. Créer le conteneur et y entrer :

   ```bash
   distrobox assemble create
   distrobox enter books-dev
   ```

3. Dans le conteneur, ajouter à la fin de `~/.bashrc` la ligne suivante, pour rendre accessibles les outils installés avec `cargo install` :

   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

4. Installer la toolchain Rust :

   ```bash
   rustup default stable
   rustup component add rust-analyzer
   ```

Le conteneur inclut aussi Helix et Starship. Leurs configurations sont personnelles et ne font pas partie du dépôt.

## Lancer le projet

Depuis le conteneur, à la racine du projet :

```bash
cargo run
```

Le service est alors accessible sur <http://localhost:3000>.

## Documentation

Le document de conception (contexte, exigences, choix techniques) se trouve dans [`docs/`](docs/).
