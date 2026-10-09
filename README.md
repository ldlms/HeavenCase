# Heaven Case

Plateforme d'ouverture de caisses en ligne pour la communauté gaming. On ouvre des caisses virtuelles thématiques, on découvre des objets de différentes raretés et on les retrouve dans son inventaire.

Linear : https://linear.app/lemedpo/project/heaven-case-0f468adeae13/overview

Documentation Bonne Pratique :https://linear.app/lemedpo/document/heaven-case-workflow-de-developpement-d67d2f677caa

Documentation Guide : https://linear.app/lemedpo/document/heaven-case-guide-linear-methodologie-agile-7e237ada3365

> **Statut :** en développement, version cible **v0.0.1**.
> Le périmètre de la V1 est volontairement limité à la boucle centrale : créer un compte, créditer son portefeuille, ouvrir une caisse, récupérer un objet.

## Sommaire

- [Fonctionnalités](#fonctionnalités)
- [Stack technique](#stack-technique)
- [Structure du dépôt](#structure-du-dépôt)
- [Démarrage](#démarrage)
  - [Prérequis](#prérequis)
- [Méthodologie](#méthodologie)
- [Avertissement](#avertissement)

## Fonctionnalités

**V1 (v0.0.1)**

- Comptes et authentification, avec vérification de l'âge (18+)
- Catalogue de caisses avec chances de drop affichées
- Portefeuille en crédits virtuels, achat de caisses
- Back-office pour gérer caisses, objets, prix et probabilités

**Plus tard (V2 et au-delà)**

Battles de caisses, upgrades, contrats, niveaux, classements, parrainage, bonus quotidiens.

## Stack technique

| Couche | Technologie |
| --- | --- |
| Front-end | React |
| Back-end | Rust |
| Gestion de projet | Linear |
| Code et revues | GitHub |

## Structure du dépôt

```
.
├── frontend/     # Application React
├── backend/      # API Rust
└── README.md
```

> À adapter selon l'organisation réelle du dépôt.

## Démarrage

### Prérequis

- Node.js (version LTS) et npm
- Rust et Cargo (via [rustup](https://rustup.rs))
- Git, configuré pour privilégier le rebase.

## Méthodologie

Organisation agile dans Linear : un projet, un milestone par version, puis la hiérarchie **Epic → Feature → User Story**. Seules les User Stories portent le code et sont estimées (échelle Fibonacci, 5 points maximum), en cycles d'une semaine.

## Avertissement

Heaven Case utilise une monnaie virtuelle, sans retrait en argent réel. Le site est réservé aux personnes majeures. Avant tout lancement public, la réglementation applicable aux jeux avec objets de valeur doit être vérifiée.

