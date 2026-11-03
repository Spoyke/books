// Mise en forme d'une exigence fonctionnelle : identifiant, titre, récit, critères
#let ef(id, titre, recit, criteres: ()) = block(breakable: false, below: 1.4em)[
  *#id : #titre* \
  #recit
  #if criteres.len() > 0 { list(..criteres) }
]

= Contexte

Un lecteur accumule au fil du temps une longue liste de livres lus et une autre, tout aussi longue, de livres qu'il aimerait lire. Sans moyen d'en garder une trace, il finit par ne plus savoir s'il a déjà lu un livre, ou par oublier celui qu'il s'était promis de lire.

// À garder si c'est ta motivation, sinon supprimer le paragraphe suivant
Des services en ligne répondent à ce besoin, mais ils hébergent les données de leurs utilisateurs sur leurs propres serveurs. Ce projet propose une alternative auto-hébergée, où l'utilisateur garde la maîtrise de ses données.

= Objectifs

Le projet consiste à créer un service web de bibliothèque personnelle : l'utilisateur y enregistre les livres qu'il a lus et ceux qu'il veut lire, avec sa note et son avis. Les livres peuvent être importés depuis Open Library ou saisis manuellement.

= Périmètre

== Version 1

- un seul utilisateur ;
- ajout de livres par import depuis Open Library ou par saisie manuelle ;
- une fiche par livre, consultable et modifiable ;
- une liste principale qui sépare les livres lus et à lire, avec tri et recherche.

== Évolutions envisagées

- plusieurs utilisateurs, chacun avec sa propre bibliothèque ;
- une note moyenne par livre, calculée à partir des notes de tous les utilisateurs qui l'ont lu ;
- la recommandation d'un livre à un autre utilisateur.

== Hors périmètre

- application mobile native (l'interface web doit en revanche rester utilisable sur téléphone) ;
- recommandations automatiques ;
- import depuis d'autres services (Goodreads, Babelio…).

= Exigences fonctionnelles

#ef(
  "EF-01", "Importer un livre depuis Open Library",
  [En tant qu'utilisateur, je veux rechercher un livre par ISBN ou par titre sur Open Library afin de l'ajouter sans tout saisir.],
  criteres: (
    [Les informations trouvées préremplissent un formulaire que je peux corriger avant d'enregistrer.],
    [Si aucun livre n'est trouvé, le service me le signale et me propose la saisie manuelle.],
  ),
)

#ef(
  "EF-02", "Ajouter un livre manuellement",
  [En tant qu'utilisateur, je veux saisir moi-même les informations d'un livre afin d'ajouter un livre absent d'Open Library.],
  criteres: (
    [Seuls le titre et l'auteur sont obligatoires.],
  ),
)

#ef(
  "EF-03", "Éviter les doublons",
  [En tant qu'utilisateur, je veux être prévenu si j'ajoute un livre déjà présent dans ma bibliothèque.],
  criteres: (
    [La détection se fait au moins sur l'ISBN quand il est renseigné.],
  ),
)

#ef(
  "EF-04", "Consulter la fiche d'un livre",
  [En tant qu'utilisateur, je veux voir toutes les informations d'un livre sur une page dédiée.],
  criteres: (
    [La fiche affiche le titre, la couverture, la description, le ou les genres, l'auteur, la date de parution, le statut de lecture, ma note et mon avis.],
    [Les champs vides n'empêchent pas l'affichage.],
  ),
)

#ef(
  "EF-05", "Modifier un livre",
  [En tant qu'utilisateur, je veux modifier les informations d'un livre depuis sa fiche afin de corriger ou compléter les données importées.],
)

#ef(
  "EF-06", "Supprimer un livre",
  [En tant qu'utilisateur, je veux retirer un livre de ma bibliothèque.],
  criteres: (
    [Une confirmation est demandée avant la suppression.],
  ),
)

#ef(
  "EF-07", "Indiquer le statut de lecture",
  [En tant qu'utilisateur, je veux indiquer si un livre est à lire ou lu afin de distinguer mes deux listes.],
  criteres: (
    [Le changement de statut se fait en une action depuis la fiche.],
  ),
)

#ef(
  "EF-08", "Noter et donner un avis",
  [En tant qu'utilisateur, je veux attribuer une note et écrire un avis sur un livre lu afin de me souvenir de ce que j'en ai pensé.],
  criteres: (
    [La note est un entier de 1 à 5.],
    [L'avis est un texte libre, facultatif.],
  ),
)

#ef(
  "EF-09", "Consulter la liste des livres",
  [En tant qu'utilisateur, je veux voir tous mes livres sur la page principale, avec une séparation visible entre les livres lus et à lire.],
)

#ef(
  "EF-10", "Trier la liste",
  [En tant qu'utilisateur, je veux trier mes livres par titre, auteur, note, date de parution ou date d'ajout.],
)

#ef(
  "EF-11", "Rechercher un livre",
  [En tant qu'utilisateur, je veux chercher un livre de ma bibliothèque par titre ou par auteur afin de savoir rapidement si je l'ai déjà lu ou noté.],
)

= Exigences non fonctionnelles

#table(
  columns: (auto, auto, 1fr),
  inset: 6pt,
  table.header([*Id*], [*Catégorie*], [*Exigence*]),
  [ENF-01], [Déploiement], [Le service fonctionne sur un Raspberry Pi, sous la forme d'un exécutable unique, avec une consommation mémoire faible.],
  [ENF-02], [Sécurité], [En v1, le service n'est accessible que depuis le réseau privé de l'utilisateur (Tailscale). Toute exposition sur Internet nécessitera une authentification.],
  [ENF-03], [Robustesse], [Le service reste utilisable si Open Library est indisponible ou lent : la saisie manuelle fonctionne toujours, et les appels à l'API ont un délai maximal.],
  [ENF-04], [Robustesse], [Les couvertures sont stockées localement : elles restent affichées même si la source disparaît.],
  [ENF-05], [Données], [Les données sont stockées dans une base locale, facile à sauvegarder.],
  [ENF-06], [Portabilité], [L'utilisateur peut exporter toute sa bibliothèque dans un format ouvert (CSV ou JSON).],
  [ENF-07], [Ergonomie], [L'interface est en français et utilisable sur téléphone comme sur ordinateur.],
  [ENF-08], [Performance], [Les pages s'affichent en moins d'une seconde sur le Pi, pour une bibliothèque de quelques milliers de livres.],
  [ENF-09], [Maintenabilité], [Les évolutions du schéma de la base sont appliquées automatiquement au démarrage, sans perte de données.],
  [ENF-10], [Évolutivité], [Le modèle de données distingue les informations d'un livre de la relation entre l'utilisateur et ce livre, pour permettre le passage au multi-utilisateur.],
  [ENF-11], [Conformité], [L'utilisation de l'API Open Library respecte ses conditions d'utilisation : le service s'identifie et n'envoie pas de requêtes en masse.],
)
