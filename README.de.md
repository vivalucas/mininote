# MiniNote

Ein leichtgewichtiger Texteditor für macOS – inspiriert vom Erlebnis des Windows 11 Editors.

[中文](README.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Français](README.fr.md) | [Русский](README.ru.md)

---

## Warum ich das gemacht habe

Ich bin von Windows zu Mac gewechselt und habe mich an vieles gewöhnt, aber der Windows 11 Editor hat mir immer gefehlt.

Nicht weil er so mächtig ist, sondern gerade weil er einfach ist. Aufmachen und schreiben, schließen ohne etwas zu verlieren, einfügen und sauberen Text bekommen. Keine Formatierung, kein Rich Text, keine „smarten" Vorschläge. Einfach ein ruhiger Ort zum Schreiben.

Ich habe auf macOS lange gesucht. Entweder zu schwerfällig (Obsidian, Typora), zu karg (der mitgelieferte TextEdit), oder Abo-basiert. Nichts war genau richtig.

Also habe ich selbst eines gebaut.

## Funktionen

- **Tab-Persistenz**
  - Einen neuen Tab erstellen und sofort schreiben, Inhalt speichert sich in Echtzeit
  - Herunterfahren, Neustart, Stromausfall – alles wird wiederhergestellt, nichts geht verloren
  - Beim Schließen des Fensters bleiben alle Tabs still erhalten, ohne Nachfrage
  - Beim Schließen eines einzelnen Tabs mit ungespeicherten Änderungen: Speichern / Nicht speichern / Abbrechen

- **Zweischichtige Speicherlogik**
  - Session-Layer: zeichnet alles in Echtzeit auf, übersteht Neustarts
  - Festplatten-Layer: schreibt nur bei explizitem Cmd+S in die Datei
  - Beide Layer arbeiten unabhängig

- **Reintext-Einfügen**
  - Einfügen ist standardmäßig Reintext, kein extra Schritt nötig
  - Von Webseiten, WeChat, PDFs kopierter Text wird automatisch von Formatierung befreit
  - Funktioniert als „Formatierungs-Reinigungsstation": Rich Text einfügen → wieder kopieren → sauberer Text

- **Optionales Markdown-Rendering**
  - Standardmäßig Reintext-Bearbeitung, Cmd+R wechselt zur gerenderten Ansicht
  - Gerenderte Ansicht ist schreibgeschützt, zurück zum Reintext zum weiteren Bearbeiten
  - .mint / .txt / .md haben jeweils unabhängige Rendering-Schalter in den Einstellungen

- **Finder-Integration**
  - Rechtsklick auf einen beliebigen Ordner im Finder, um eine neue .mint-Datei zu erstellen
  - Natives macOS Quick Look – Datei auswählen und Leertaste drücken zur Vorschau
  - .mint-Dateien werden standardmäßig mit MiniNote geöffnet

- **Sonstiges**
  - Statusleiste zeigt Zeile/Spalte, Zeichenzahl, Kodierung, Zeilenumbruch, Rendermodus
  - Drei Dateiformate: .mint (Standard), .txt, .md – Umwandlung über Speichern unter
  - Themawechsel: Hell / Dunkel / Systemeinstellung
  - GitHub-Updates über die Einstellungen prüfen

## Layout

```
+-------------------------------------------+
| Menüleiste                                |
+-------------------------------------------+
| [Ohne Titel]  [notes.mint]  [ideas.md]  [+] |
+-------------------------------------------+
|                                           |
|              Editorbereich                |
|                                           |
+-------------------------------------------+
| Statusleiste (Z/Sp | Zeichen | UTF-8 | LF | Text) |
+-------------------------------------------+
```

## Tastenkürzel

| Funktion | Tastenkürzel |
|----------|-------------|
| Neu | `Cmd+N` |
| Öffnen | `Cmd+O` |
| Speichern | `Cmd+S` |
| Speichern unter | `Cmd+Shift+S` |
| Tab schließen | `Cmd+W` |
| Rückgängig / Wiederherstellen | `Cmd+Z` / `Cmd+Shift+Z` |
| Suchen | `Cmd+F` |
| Suchen & Ersetzen | `Cmd+Option+F` |
| Markdown umschalten | `Cmd+R` |
| Einstellungen | `Cmd+,` |

## Unterstützte Formate

| Format | Beschreibung |
|--------|-------------|
| .mint | MiniNote-eigenes Format, Standard für neue Dateien. Reintext + leichtgewichtige Statusinfo (Cursorposition, Renderstatus) |
| .txt | Standard-Reintext, kompatibel mit anderen Editoren |
| .md | Markdown-Format |

Alle drei sind im Kern Reintext. Die Umwandlung erfolgt durch einfaches Ändern der Erweiterung.

## Systemanforderungen

- macOS 26 (Tahoe) oder neuer
- Apple Silicon Mac (M-Serie Chips)

## Installation

**Option 1: DMG-Installer**

1. Gehen Sie zur [Releases](../../releases)-Seite und laden Sie die neueste `MiniNote-[Version].dmg` herunter
2. Öffnen Sie die DMG und ziehen Sie MiniNote in den Applications-Ordner
3. Beim ersten Start kann macOS „App ist beschädigt" oder „Entwickler kann nicht überprüft werden" anzeigen – dies ist das normale Gatekeeper-Verhalten für unsignierte Apps. Führen Sie dies im Terminal aus, um die Quarantäne-Flagge zu entfernen:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   Dann Doppelklick zum Starten. Oder Rechtsklick → Öffnen → Im Dialog „Öffnen" klicken.

**Option 2: ZIP-Archiv**

1. Gehen Sie zur [Releases](../../releases)-Seite und laden Sie die neueste `MiniNote-[Version].zip` herunter und entpacken Sie sie
2. Verschieben Sie `MiniNote.app` in den Applications-Ordner
3. Im Terminal ausführen:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**Option 3: Aus dem Quellcode kompilieren (kein Signatur-Workaround nötig)**

1. Dieses Repository klonen
2. `MiniNote.xcodeproj` in Xcode öffnen
3. Unter **Signing & Capabilities** Ihr eigenes Entwicklerkonto auswählen
4. `Cmd+R` zum Ausführen – Xcode übernimmt die Signierung automatisch

## Verwendung

- **Neuer Tab**: `Cmd+N` erstellt ein Scratch-Dokument, sofort mit dem Schreiben beginnen
- **Datei öffnen**: `Cmd+O` öffnet .mint / .txt / .md Dateien von der Festplatte
- **Speichern**: `Cmd+S` speichert den aktuellen Tab auf die Festplatte; Scratch-Dokumente öffnen den „Speichern unter"-Dialog
- **Speichern unter**: `Cmd+Shift+S` speichert in einem anderen Format (.mint / .txt / .md)
- **Rendering umschalten**: `Cmd+R` wechselt zwischen Reintext-Bearbeitung und Markdown-Ansicht
- **Suchen & Ersetzen**: `Cmd+F` zum Suchen, `Cmd+Option+F` zum Suchen und Ersetzen
- **Tabs neu anordnen**: Tabs per Drag & Drop verschieben
- **Tab schließen**: `Cmd+W`, fragt nach dem Speichern bei ungespeicherten Änderungen
- **Updates prüfen**: In den Einstellungen (`Cmd+,`) gibt es eine Schaltfläche „Nach Updates suchen"

## FAQ

**Wo werden Scratch-Dokumente gespeichert?**

Im Verzeichnis `~/Library/Application Support/MiniNote/sessions/`. Jedes Scratch-Dokument ist eine separate Datei, plus eine `session.json` mit Tab-Reihenfolge und Metadaten.

**Was ist der Unterschied zwischen .mint und .txt?**

Im Kern identisch – beides Reintext. Der einzige Unterschied ist, dass .mint zusätzlich Cursorposition und Renderstatus speichert. Beide können beliebig über „Speichern unter" umgewandelt werden, ohne den Inhalt zu verlieren.

**Wird Syntaxhervorhebung unterstützt?**

Nein. MiniNote ist ein Reintext-Editor. Das Markdown-Rendering verwendet die systemeigene AttributedString für grundlegende Überschriften, Listen, Fettdruck usw. Keine Code-Syntaxhervorhebung.

**Wie unterscheidet sich MiniNote von TextEdit / CotEditor / BBEdit?**

MiniNote's Kernphilosophie ist: Tab-Persistenz (übersteht Neustarts) + zweischichtige Speicherlogik (Trennung von Scratch und Festplatte). TextEdit unterstützt keine Tab-Persistenz. CotEditor bietet mehr Funktionen, hat aber keinen solchen Mechanismus. BBEdit ist zu schwerfällig. MiniNote macht nur eine Sache, aber richtig.

**Wird Cloud-Synchronisation unterstützt?**

Nein, und das wird auch nie der Fall sein. Alle Daten werden lokal gespeichert, vollständig offline.

## Entwicklung

Tech-Stack: Swift 6 + SwiftUI + NSTextView (TextKit), keine Drittanbieter-Abhängigkeiten.

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# Cmd+B zum Kompilieren
```

## Lizenz

MIT License
