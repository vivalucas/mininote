# MiniNote

Un éditeur de texte léger pour macOS, inspiré de l'expérience du Bloc-notes Windows 11.

[中文](README.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [繁體中文](README.zh-TW.md) | [Português](README.pt-BR.md) | [Italiano](README.it.md) | [Русский](README.ru.md)

---

## Pourquoi je l'ai fait

Je suis passé de Windows à Mac et me suis habitué à la plupart des choses, mais le Bloc-notes Windows 11 m'a manqué.

Pas parce qu'il est puissant – justement parce qu'il est simple. Ouvrir et écrire, fermer sans rien perdre, coller et obtenir du texte propre. Pas de formatage, pas de texte enrichi, pas de suggestions « intelligentes ». Juste un endroit tranquille pour écrire.

J'ai cherché longtemps sur macOS. Soit trop lourd (Obsidian, Typora), soit trop basique (TextEdit), soit par abonnement. Rien ne convenait parfaitement.

Alors j'en ai créé un moi-même.

## Fonctionnalités

- **Persistance des onglets**
  - Créez un nouvel onglet et commencez à écrire, le contenu s'enregistre en temps réel
  - Arrêt, redémarrage, coupure de courant – tout est restauré, rien n'est perdu
  - La fermeture de la fenêtre conserve silencieusement tous les onglets, sans aucun message
  - La fermeture d'un onglet avec des modifications non enregistrées affiche Enregistrer / Ne pas enregistrer / Annuler

- **Logique de sauvegarde à deux niveaux**
  - Couche Session : enregistre tout en temps réel, survit aux redémarrages
  - Couche disque : n'écrit dans le fichier que lors d'un Cmd+S explicite
  - Les deux couches fonctionnent indépendamment

- **Collage en texte brut**
  - Le collage est du texte brut par défaut, aucune étape supplémentaire nécessaire
  - Le texte copié depuis des pages web, WeChat, PDF est automatiquement dépouillé de son formatage
  - Fonctionne comme une « station de nettoyage de format » : coller du texte enrichi → recopier → texte propre

- **Rendu Markdown (optionnel)**
  - Édition en texte brut par défaut, Cmd+R pour basculer vers la vue rendue
  - La vue rendue est en lecture seule, revenir au texte brut pour continuer l'édition
  - .mint / .txt / .md ont chacun leur propre interrupteur de rendu dans les Préférences

- **Intégration Finder**
  - Clic droit sur n'importe quel dossier dans le Finder pour créer un nouveau fichier .mint
  - Support natif Quick Look de macOS – sélectionner un fichier et appuyer sur Espace pour prévisualiser
  - Les fichiers .mint s'ouvrent avec MiniNote par défaut

- **Autres**
  - La barre d'état affiche ligne/colonne, nombre de caractères, encodage, fin de ligne, mode de rendu
  - Trois formats de fichier : .mint (par défaut), .txt, .md – conversion via Enregistrer sous
  - Changement de thème : Clair / Sombre / Suivre le système
  - Vérification des mises à jour GitHub depuis les Préférences

## Disposition

```
+-------------------------------------------+
| Barre de menus                            |
+-------------------------------------------+
| [Sans titre]  [notes.mint]  [ideas.md]  [+] |
+-------------------------------------------+
|                                           |
|              Zone d'édition               |
|                                           |
+-------------------------------------------+
| Barre d'état (Lg/Col | Car. | UTF-8 | LF | Texte) |
+-------------------------------------------+
```

## Raccourcis clavier

| Fonction | Raccourci |
|----------|----------|
| Nouveau | `Cmd+N` |
| Ouvrir | `Cmd+O` |
| Enregistrer | `Cmd+S` |
| Enregistrer sous | `Cmd+Shift+S` |
| Fermer l'onglet | `Cmd+W` |
| Annuler / Rétablir | `Cmd+Z` / `Cmd+Shift+Z` |
| Rechercher | `Cmd+F` |
| Rechercher & Remplacer | `Cmd+Option+F` |
| Basculer Markdown | `Cmd+R` |
| Préférences | `Cmd+,` |

## Formats pris en charge

| Format | Description |
|--------|-------------|
| .mint | Format natif MiniNote, par défaut pour les nouveaux fichiers. Texte brut + informations d'état légères (position du curseur, état du rendu) |
| .txt | Texte brut standard, compatible avec d'autres éditeurs |
| .md | Format Markdown |

Les trois sont fondamentalement du texte brut. La conversion entre eux revient simplement à changer l'extension.

## Configuration requise

- macOS 26 (Tahoe) ou ultérieur
- Apple Silicon Mac (puces M)

## Installation

**Option 1 : Installateur DMG**

1. Allez sur la page [Releases](../../releases) et téléchargez le dernier `MiniNote-[version].dmg`
2. Ouvrez le DMG et faites glisser MiniNote dans le dossier Applications
3. Au premier lancement, macOS peut afficher « l'application est endommagée » ou « impossible de vérifier le développeur » – c'est le comportement normal de Gatekeeper pour les applications non signées. Exécutez ceci dans le Terminal pour supprimer le drapeau de quarantaine :
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   Puis double-cliquez pour lancer. Ou clic droit → Ouvrir → cliquez sur Ouvrir dans la boîte de dialogue.

**Option 2 : Archive ZIP**

1. Allez sur la page [Releases](../../releases) et téléchargez le dernier `MiniNote-[version].zip`, puis décompressez
2. Déplacez `MiniNote.app` dans le dossier Applications
3. Exécutez dans le Terminal :
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**Option 3 : Compiler depuis les sources (aucun contournement de signature nécessaire)**

1. Clonez ce dépôt
2. Ouvrez `MiniNote.xcodeproj` dans Xcode
3. Dans **Signing & Capabilities**, sélectionnez votre propre compte développeur
4. `Cmd+R` pour exécuter – Xcode gère la signature automatiquement

## Utilisation

- **Nouvel onglet** : `Cmd+N` crée un document temporaire, commencez à écrire immédiatement
- **Ouvrir un fichier** : `Cmd+O` ouvre les fichiers .mint / .txt / .md du disque
- **Enregistrer** : `Cmd+S` enregistre l'onglet courant sur le disque ; les documents temporaires ouvrent la boîte de dialogue Enregistrer sous
- **Enregistrer sous** : `Cmd+Shift+S` enregistre dans un format différent (.mint / .txt / .md)
- **Basculer le rendu** : `Cmd+R` alterne entre l'édition en texte brut et la vue Markdown rendue
- **Rechercher & Remplacer** : `Cmd+F` pour chercher, `Cmd+Option+F` pour chercher et remplacer
- **Réorganiser les onglets** : faites glisser les onglets pour les réorganiser
- **Fermer l'onglet** : `Cmd+W`, demande de confirmation si des modifications ne sont pas enregistrées
- **Vérifier les mises à jour** : les Préférences (`Cmd+,`) ont un bouton « Vérifier les mises à jour »

## FAQ

**Où sont stockés les documents temporaires ?**

Dans le répertoire `~/Library/Application Support/MiniNote/sessions/`. Chaque document temporaire est un fichier distinct, plus un fichier `session.json` qui enregistre l'ordre des onglets et les métadonnées.

**Quelle est la différence entre .mint et .txt ?**

Fondamentalement identiques – les deux sont du texte brut. La seule différence est que .mint enregistre également la position du curseur et l'état du rendu. Vous pouvez convertir librement entre les deux via Enregistrer sous sans perdre de contenu.

**Est-ce que la coloration syntaxique est prise en charge ?**

Non. MiniNote est un éditeur de texte brut. Le rendu Markdown utilise l'AttributedString du système pour les titres, listes, gras, etc. de base. Pas de coloration syntaxique pour le code.

**En quoi diffère-t-il de TextEdit / CotEditor / BBEdit ?**

La philosophie fondamentale de MiniNote est : persistance des onglets (survit aux redémarrages) + logique de sauvegarde à deux niveaux (séparation temporaire/disque). TextEdit ne supporte pas la persistance des onglets. CotEditor est plus riche en fonctionnalités mais n'a pas ce mécanisme. BBEdit est trop lourd. MiniNote fait une seule chose, mais bien.

**Est-ce que la synchronisation cloud est prise en charge ?**

Non, et elle ne le sera jamais. Toutes les données sont stockées localement, entièrement hors ligne.

## Développement

Stack technique : Swift 6 + SwiftUI + NSTextView (TextKit), zéro dépendance tierce.

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# Cmd+B pour compiler
```

## Licence

MIT License
