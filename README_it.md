[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | **Italiano** | [Русский](README_ru.md)

# MiniNote

[![GitHub Release](https://img.shields.io/github/v/release/vivalucas/mininote?style=flat-square)](https://github.com/vivalucas/mininote/releases) [![License](https://img.shields.io/github/license/vivalucas/mininote?style=flat-square)](LICENSE) [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows-lightgrey?style=flat-square)]()

MiniNote è un'app per note desktop local-first. Aprila, scrivi subito, tieni tutto sulla tua macchina e fissa le note sul desktop quando ti servono. Nessun account, nessuna sincronizzazione cloud, nessun flusso pesante da knowledge base.

## Technology Stack

- **Framework**: Tauri 2 (Rust)
- **Frontend**: React 19 + TypeScript
- **Styling**: TailwindCSS
- **Performance**: Uncontrolled editor architecture with debounced rendering for zero-lag typing.

## A cosa serve

- Catturare idee, comandi, attività, frammenti di riunioni o promemoria di lavoro/gioco.
- Tenere testo riutilizzabile fissato sul desktop per leggerlo o copiarlo rapidamente.
- Aprire una piccola finestra di nota senza interrompere il lavoro in corso.
- Organizzare bozze locali con categorie semplici.
- Scrivere Markdown leggero e vedere in anteprima titoli, elenchi, citazioni e blocchi di codice.

## Funzionalità principali

- **Libreria locale di note**: note, categorie e impostazioni sono salvate localmente; non devi scegliere prima un percorso file.
- **Note rapide**: apri una piccola finestra dalla tray o con una scorciatoia globale; può apparire vicino al cursore.
- **Riquadri desktop**: fissa una nota sullo schermo con colori personalizzati e rendering Markdown opzionale.
- **Anteprima Markdown**: utile per testo strutturato quotidiano, senza voler essere un IDE Markdown completo.
- **Importazione ed esportazione**: importa un singolo file o una cartella come categoria; supporta `.mint`, `.md`, `.markdown` e `.txt`.
- **Protezione della sincronizzazione con il file sorgente**: i file importati ed esportati possono mantenere un collegamento alla sorgente; MiniNote controlla eventuali modifiche esterne prima di scrivere di nuovo.
- **Aspetto configurabile**: tema, dimensione del carattere, immagine di sfondo, scorciatoie e chiusura nella tray sono regolabili.

## Formati supportati

| Formato             | Uso                                                    |
| ------------------- | ------------------------------------------------------ |
| `.mint`             | Tipo di documento predefinito di MiniNote; testo UTF-8 |
| `.md` / `.markdown` | Documento Markdown                                     |
| `.txt`              | File di testo semplice standard                        |

Tutti i file supportati possono essere aperti in un normale editor di testo. MiniNote non scrive metadati privati nel corpo del file.

## Installazione e aggiornamento

Le build ufficiali sono pubblicate su [GitHub Releases](https://github.com/vivalucas/mininote/releases). A partire dalla versione 1.0.0, gli asset ufficiali di release sono forniti solo per Windows e macOS. Il supporto Linux resta nel codice sorgente, ma per ora non viene pubblicato alcun pacchetto Linux ufficiale.

### Windows

- Installer: `mininote-<version>-windows-x64-setup.exe`
- Build portatile: `mininote-<version>-windows-x64.exe`

Usa l'installer per l'uso regolare. Usa la build portatile se vuoi provare MiniNote temporaneamente o eseguirlo da una cartella fissa.

### macOS

Gli utenti Apple Silicon devono scaricare `mininote-<version>-macos-arm64.dmg`, aprirlo e trascinare `MiniNote.app` in `Applications`.

La build macOS al momento non è firmata formalmente. Se macOS dice che l'app non può essere aperta, è danneggiata o proviene da uno sviluppatore non identificato, verifica prima che il file provenga dalla pagina Release di questo progetto, poi esegui:

```bash
xattr -cr /Applications/MiniNote.app
```

Apri di nuovo l'app dopo il comando. Per aggiornare, chiudi MiniNote, scarica il nuovo DMG e sostituisci la vecchia app in `Applications`.

### Linux

MiniNote 1.0.0 non distribuisce pacchetti Linux ufficiali. Se ti serve una build Linux, compilala dal codice sorgente; la configurazione di pacchettizzazione Linux resta nel repository.

## Dove sono salvati i dati

MiniNote non carica le note e non offre sincronizzazione cloud. Note, impostazioni e dati di indice sono archiviati per impostazione predefinita nella cartella `MiniNote` della directory dati applicazione del sistema. Se `MININOTE_DATA_DIR` è impostato, MiniNote usa quella directory.

## Compilare dal codice sorgente

Servono Node.js, Rust e le dipendenze di sistema richieste da Tauri.

```bash
npm ci
npm run tauri build
```

Modalità sviluppo:

```bash
npm run tauri dev
```

## Limiti

- Nessun account e nessuna sincronizzazione cloud.
- Nessun editor rich text.
- Nessun IDE Markdown completo.
- Nessuna knowledge base complessa, backlink o sistema collaborativo.

MiniNote punta a rimanere leggero, veloce, locale e comodo.

## Licenza

MIT License
