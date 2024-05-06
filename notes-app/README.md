# Application de Gestion de Notes

Développée en Rust, cette application de bureau offre une gestion locale des notes à travers deux systèmes de stockage : fichiers texte et base de données SQLite. Elle propose des fonctionnalités complètes de création, lecture, mise à jour et suppression de notes.

## Fonctionnalités

- **Créer des notes :** Permet d'ajouter de nouvelles notes à votre base de données ou fichier local.
- **Lire des notes :** Consultez toutes vos notes enregistrées.
- **Mettre à jour des notes :** Mettez à jour le contenu de vos notes.
- **Supprimer des notes :** Effacez les notes.

## Technologies Utilisées

- **Rust :** Le langage de programmation choisi pour le développement.
- **SQLite :** Utilisé pour la gestion des notes dans une base de données.
- **Tauri :** Un kit de développement pour créer des applications de bureau en utilisant des technologies web.

## Prérequis

Pour exécuter cette application, vous aurez besoin de :

- **Rust**
- **Node.js**
- **Tauri** 

## Configuration et Installation

Suivez ces étapes pour configurer et lancer l'application :

### 1. Clonage du dépôt

```bash
git clone https://github.com/najlaelambaraa/rust-notes-app

```
### 2. Accédez au répertoire du projet:
```
cd rust-notes-app
```
## Structure de Projet

### Dossier principal `src`
- **src/**
  - **dataBase.js** : Contient les fonctions de gestion de la base de données.
  - **index.html** : Le fichier HTML principal de l'interface utilisateur.
  - **main.js** : Le script JavaScript principal qui gère la logique frontend.
  - **styles.css** : Fichier CSS pour le style de l'interface utilisateur.

### Dossier Tauri `src-tauri`
- **src-tauri/**
  - **src/**
    - **command.rs** : Fichiers Rust pour définir les commandes Tauri.
    - **main.rs** : Point d'entrée principal de l'application Tauri.
    - **noteFile.rs** : Gère les opérations liées aux notes.
  - **notes.db** : Base de données SQLite pour stocker les notes.
  - **target/**
    - **release/** : Contient les binaires compilés en mode release.

### Notes
- **`src/`** : Contient tous les fichiers relatifs à l'interface utilisateur, comme le HTML, le CSS, et le JavaScript.
- **`src-tauri/`** : Contient tous les fichiers spécifiques à Tauri, y compris la configuration de Rust et les fichiers source.
- **`target/release/`** : Dossier généré par le système de build Rust, contenant les exécutables compilés pour la distribution.

## Utilisation

### Création de Notes

Pour créer une note, ouvrez l'application et accédez à la section de création de notes. Entrez le titre et le contenu de votre note et choisissez de la sauvegarder localement ou dans la base de données.

### Exportation en PDF

Pour exporter des notes en PDF, naviguez vers la section 'Export PDF' de l'application, sélectionnez les notes à exporter, et cliquez sur 'Exporter en PDF'.

## Configuration

Les configurations spécifiques à l'application peuvent être ajustées dans le fichier `tauri.conf.json`.

## Auteur

Najlae LAMBARAA