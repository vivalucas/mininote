[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | **Français** | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

[![GitHub Release](https://img.shields.io/github/v/release/vivalucas/mininote?style=flat-square)](https://github.com/vivalucas/mininote/releases) [![License](https://img.shields.io/github/license/vivalucas/mininote?style=flat-square)](LICENSE) [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey?style=flat-square)]()

MiniNote est une application de notes de bureau locale avant tout. Ouvrez-la, écrivez immédiatement, gardez tout sur votre machine et épinglez des notes sur le bureau quand vous en avez besoin. Pas de compte, pas de synchronisation cloud, pas de workflow lourd de base de connaissances.

## Technology Stack

- **Framework**: Tauri 2 (Rust)
- **Frontend**: React 19 + TypeScript
- **Styling**: TailwindCSS
- **Performance**: Uncontrolled editor architecture with debounced rendering for zero-lag typing.

## À quoi elle sert

- Capturer des idées, commandes, tâches, fragments de réunion ou rappels de travail/jeu.
- Garder du texte réutilisable épinglé sur le bureau pour le lire ou le copier rapidement.
- Ouvrir une petite fenêtre de note sans interrompre votre tâche en cours.
- Organiser des brouillons locaux avec des catégories simples.
- Écrire du Markdown léger et prévisualiser titres, listes, citations et blocs de code.

## Fonctionnalités principales

- **Bibliothèque locale de notes** : notes, catégories et paramètres sont stockés localement ; vous n'avez pas besoin de choisir d'abord un chemin de fichier.
- **Notes rapides** : ouvrez une petite fenêtre depuis la zone de notification ou un raccourci global ; elle peut apparaître près du curseur.
- **Tuiles de bureau** : épinglez une note à l'écran avec des couleurs personnalisées et un rendu Markdown optionnel.
- **Aperçu Markdown** : utile pour le texte structuré du quotidien, sans chercher à devenir un IDE Markdown complet.
- **Importation et exportation** : importez un fichier seul ou un dossier comme catégorie ; prend en charge `.mint`, `.md`, `.markdown` et `.txt`.
- **Protection de synchronisation avec le fichier source** : les fichiers importés et exportés peuvent conserver un lien source ; MiniNote vérifie les changements externes avant de réécrire.
- **Apparence configurable** : thème, taille de police, image d'arrière-plan, raccourcis et fermeture dans la zone de notification sont réglables.

## Formats pris en charge

| Format              | Utilisation                                           |
| ------------------- | ----------------------------------------------------- |
| `.mint`             | Type de document par défaut de MiniNote ; texte UTF-8 |
| `.md` / `.markdown` | Document Markdown                                     |
| `.txt`              | Fichier texte brut standard                           |

Tous les fichiers pris en charge peuvent être ouverts dans un éditeur de texte normal. MiniNote n'écrit pas de métadonnées privées dans le corps du fichier.

## Installation et mise à jour

Les builds officiels sont publiés sur [GitHub Releases](https://github.com/vivalucas/mininote/releases). À partir de la version 1.0.0, les fichiers officiels de release sont fournis uniquement pour Windows et macOS. Le support Linux reste dans le code source, mais aucun paquet Linux officiel n'est publié pour le moment.

### Windows

- Installateur : `mininote-<version>-windows-x64-setup.exe`
- Version portable : `mininote-<version>-windows-x64.exe`

Utilisez l'installateur pour un usage régulier. Utilisez la version portable pour essayer MiniNote temporairement ou l'exécuter depuis un dossier fixe.

### macOS

Les utilisateurs Apple Silicon doivent télécharger `mininote-<version>-macos-arm64.dmg`, l'ouvrir, puis faire glisser `MiniNote.app` dans `Applications`.

Le build macOS n'est actuellement pas signé officiellement. Si macOS indique que l'app ne peut pas être ouverte, qu'elle est endommagée ou qu'elle provient d'un développeur non identifié, vérifiez d'abord que le fichier vient bien de la page Release du projet, puis exécutez :

```bash
xattr -cr /Applications/MiniNote.app
```

Ouvrez ensuite l'application à nouveau. Pour mettre à jour, quittez MiniNote, téléchargez le nouveau DMG et remplacez l'ancienne app dans `Applications`.

### Linux

MiniNote 1.0.0 ne fournit pas de paquets Linux officiels. Si vous avez besoin d'une version Linux, compilez-la depuis les sources ; la configuration de paquetage Linux reste dans le dépôt.

## Où vivent les données

MiniNote n'envoie pas vos notes et ne fournit pas de synchronisation cloud. Les notes, paramètres et données d'index sont stockés par défaut dans le dossier `MiniNote` du répertoire de données d'application du système. Si `MININOTE_DATA_DIR` est défini, MiniNote utilise ce dossier à la place.

## Compiler depuis les sources

Vous avez besoin de Node.js, Rust et des dépendances système requises par Tauri.

```bash
npm ci
npm run tauri build
```

Mode développement :

```bash
npm run tauri dev
```

## Limites

- Pas de comptes ni de synchronisation cloud.
- Pas d'éditeur de texte enrichi.
- Pas d'IDE Markdown complet.
- Pas de base de connaissances complexe, de backlinks ou de collaboration.

MiniNote vise à rester léger, rapide, local et pratique.

## Licence

MIT License
