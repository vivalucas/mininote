[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | **Deutsch** | [Français](README_fr.md) | [Español](README_es.md) | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

MiniNote ist eine lokale Desktop-Notiz-App. Öffnen, sofort schreiben, alles auf dem eigenen Rechner behalten und Notizen bei Bedarf auf dem Desktop anheften. Kein Konto, keine Cloud-Synchronisierung, kein schwerer Knowledge-Base-Workflow.

## Wofür MiniNote gedacht ist

- Ideen, Befehle, Aufgaben, Besprechungsnotizen oder Arbeits- und Spielhinweise schnell festhalten.
- Wiederverwendbare Texte auf dem Desktop anheften, um sie schnell zu lesen oder zu kopieren.
- Ein kleines Notizfenster öffnen, ohne die aktuelle Arbeit zu unterbrechen.
- Lokale Entwürfe mit einfachen Kategorien organisieren.
- Leichtes Markdown schreiben und Überschriften, Listen, Zitate sowie Codeblöcke schnell prüfen.

## Hauptfunktionen

- **Lokale Notizbibliothek**: Notizen, Kategorien und Einstellungen werden lokal gespeichert; Sie müssen nicht zuerst einen Dateipfad auswählen.
- **Schnellnotizen**: Öffnen Sie ein kleines Notizfenster über den Infobereich oder ein globales Tastenkürzel; es kann in der Nähe des Mauszeigers erscheinen.
- **Desktop-Kacheln**: Heften Sie eine Notiz mit eigenen Farben und optionalem Markdown-Rendering auf dem Bildschirm an.
- **Markdown-Vorschau**: Nützlich für alltägliche strukturierte Texte, nicht als vollständige Markdown-IDE gedacht.
- **Import und Export**: Importieren Sie einzelne Dateien oder einen Ordner als Kategorie; unterstützt `.mint`, `.md`, `.markdown` und `.txt`.
- **Schutz beim Abgleich mit Quelldateien**: Importierte und exportierte Dateien können als verknüpfte Quelle erhalten bleiben; MiniNote prüft externe Änderungen, bevor es zurückschreibt.
- **Anpassbares Erscheinungsbild**: Thema, Schriftgröße, Hintergrundbild, Tastenkürzel und Verhalten beim Schließen in den Infobereich sind konfigurierbar.

## Unterstützte Formate

| Format              | Verwendung                                    |
| ------------------- | --------------------------------------------- |
| `.mint`             | MiniNotes Standarddokumenttyp; UTF-8-Klartext |
| `.md` / `.markdown` | Markdown-Dokument                             |
| `.txt`              | Standard-Textdatei                            |

Alle unterstützten Dateien können weiterhin in einem normalen Texteditor geöffnet werden. MiniNote schreibt keine privaten Metadaten in den Dateiinhalt.

## Installieren und Aktualisieren

Offizielle Builds werden auf [GitHub Releases](https://github.com/vivalucas/mininote/releases) veröffentlicht. Ab 1.0.0 werden offizielle Release-Dateien nur für Windows und macOS bereitgestellt. Linux-Unterstützung bleibt im Quellcode erhalten, aber derzeit wird kein offizielles Linux-Paket veröffentlicht.

### Windows

- Installer: `mininote-<version>-windows-x64-setup.exe`
- Portable Version: `mininote-<version>-windows-x64.exe`

Verwenden Sie den Installer für den regulären Einsatz. Die portable Version eignet sich zum Ausprobieren oder zum direkten Start aus einem festen Ordner.

### macOS

Nutzer von Apple Silicon laden `mininote-<version>-macos-arm64.dmg` herunter, öffnen es und ziehen `MiniNote.app` nach `Applications`.

Der macOS-Build ist derzeit nicht offiziell signiert. Wenn macOS meldet, dass die App nicht geöffnet werden kann, beschädigt ist oder von einem nicht verifizierten Entwickler stammt, prüfen Sie zuerst, dass die Datei von der Release-Seite dieses Projekts stammt, und führen Sie dann aus:

```bash
xattr -cr /Applications/MiniNote.app
```

Öffnen Sie die App anschließend erneut. Zum Aktualisieren beenden Sie MiniNote, laden die neue DMG-Datei herunter und ersetzen die alte App in `Applications`.

### Linux

MiniNote 1.0.0 enthält keine offiziellen Linux-Pakete. Wenn Sie eine Linux-Version benötigen, bauen Sie sie bitte aus dem Quellcode. Die Linux-Paketkonfiguration bleibt im Repository erhalten.

## Wo die Daten liegen

MiniNote lädt keine Notizen hoch und bietet keine Cloud-Synchronisierung. Notizen, Einstellungen und Indexdaten werden standardmäßig im Ordner `MiniNote` im App-Datenverzeichnis des Systems gespeichert. Wenn `MININOTE_DATA_DIR` gesetzt ist, verwendet MiniNote dieses Verzeichnis.

## Aus dem Quellcode bauen

Sie benötigen Node.js, Rust und die von Tauri benötigten Systemabhängigkeiten.

```bash
npm ci
npm run tauri build
```

Entwicklungsmodus:

```bash
npm run tauri dev
```

## Grenzen

- Keine Konten und keine Cloud-Synchronisierung.
- Kein Rich-Text-Editor.
- Keine vollständige Markdown-IDE.
- Keine komplexe Wissensdatenbank, Backlinks oder Zusammenarbeit.

MiniNote soll leicht, schnell, lokal und bequem bleiben.

## Lizenz

MIT License
