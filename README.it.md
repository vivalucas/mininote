# MiniNote

Un editor di testo leggero per macOS, ispirato all'esperienza del Blocco note di Windows 11.

[中文](README.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Español](README.es.md) | [繁體中文](README.zh-TW.md) | [Português](README.pt-BR.md) | [Русский](README.ru.md)

---

## Perché l'ho creato

Sono passato da Windows a Mac e mi sono abituato alla maggior parte delle cose, ma mi mancava sempre il Blocco note di Windows 11.

Non perché sia potente -- proprio perché è semplice. Aprilo e scrivi, chiudilo senza perdere nulla, incolla e ottieni testo pulito. Nessuna formattazione, nessun rich text, nessun suggerimento "intelligente". Solo un posto tranquillo per scrivere.

Ho cercato a lungo su macOS. O troppo pesante (Obsidian, Typora), troppo essenziale (TextEdit integrato), o basato su abbonamento. Niente che fosse proprio quello che serviva.

Così ho creato il mio.

## Funzionalità

- **Persistenza delle schede**
  - Crea una nuova scheda e inizia a scrivere, il contenuto si salva automaticamente in tempo reale
  - Spegni, riavvia, interruzioni di corrente -- tutto viene ripristinato, nulla viene perso
  - Chiudere la finestra preserva silenziosamente tutte le schede, nessun avviso
  - Chiudere una singola scheda con modifiche non salvate mostra Salva / Non salvare / Annulla

- **Logica di salvataggio a due livelli**
  - Il livello sessione registra tutto in tempo reale, sopravvive ai riavvii
  - Il livello disco scrive sul file solo con Cmd+S esplicito
  - I due livelli operano indipendentemente

- **Incolla come testo semplice**
  - Incolla è testo semplice per impostazione predefinita, nessun passaggio extra
  - Il testo copiato da pagine web, WeChat, PDF viene automaticamente privato della formattazione
  - Funziona come "stazione di pulizia formato": incolla rich text, copialo, ottieni testo pulito

- **Rendering Markdown opzionale**
  - Modifica testo semplice per impostazione predefinita, Cmd+R per alternare visualizzazione renderizzata
  - La visualizzazione renderizzata è di sola lettura, torna al testo semplice per continuare a modificare
  - .mint / .txt / .md hanno interruttori di rendering indipendenti nelle Preferenze

- **Integrazione con Finder**
  - Clic destro su qualsiasi cartella nel Finder per creare un nuovo file .mint
  - Supporto nativo Quick Look di macOS -- seleziona un file e premi Spazio per anteprima
  - I file .mint si aprono con MiniNote per impostazione predefinita

- **Altro**
  - Barra di stato mostra riga/colonna, conteggio caratteri, codifica, fine riga, modalità rendering
  - Tre formati file: .mint (predefinito), .txt, .md -- converti tramite Salva come
  - Cambio tema: Chiaro / Scuro / Segui Sistema
  - Controlla aggiornamenti GitHub dalle Preferenze

## Scorciatoie da tastiera

| Funzione | Scorciatoia |
|----------|----------|
| Nuovo | `Cmd+N` |
| Apri | `Cmd+O` |
| Salva | `Cmd+S` |
| Salva come | `Cmd+Shift+S` |
| Chiudi scheda | `Cmd+W` |
| Annulla / Ripeti | `Cmd+Z` / `Cmd+Shift+Z` |
| Trova | `Cmd+F` |
| Trova e sostituisci | `Cmd+Option+F` |
| Alterna Markdown | `Cmd+R` |
| Preferenze | `Cmd+,` |

## Formati supportati

| Formato | Descrizione |
|--------|-------------|
| .mint | Formato nativo MiniNote, predefinito per nuovi file. Testo semplice + informazioni stato leggere (posizione cursore, stato rendering) |
| .txt | Testo semplice standard, compatibile con altri editor |
| .md | Formato Markdown |

Tutti e tre sono testo semplice alla base. Convertire tra loro è solo cambiare l'estensione.

## Requisiti

- macOS 26 (Tahoe) o successivo
- Mac con Apple Silicon (chip serie M)

## Installazione

**Opzione 1: Installatore DMG**

1. Vai alla pagina [Releases](../../releases) e scarica l'ultimo `MiniNote-[versione].dmg`
2. Apri il DMG e trascina MiniNote nella cartella Applicazioni
3. Al primo avvio, macOS potrebbe dire "l'app è danneggiata" o "impossibile verificare lo sviluppatore" -- questo è comportamento normale di Gatekeeper per app non firmate. Esegui questo nel Terminal per rimuovere il flag di quarantena:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   Poi doppio clic per avviare; o clic destro, Apri, poi clic su Apri nel dialogo.

**Opzione 2: Archivio ZIP**

1. Vai alla pagina [Releases](../../releases) e scarica l'ultimo `MiniNote-[versione].zip`, poi decomprimi
2. Sposta `MiniNote.app` nella cartella Applicazioni
3. Esegui nel Terminal:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**Opzione 3: Compila da sorgente (nessuna soluzione di firma necessaria)**

1. Clona questo repository
2. Apri `MiniNote.xcodeproj` in Xcode
3. In **Signing & Capabilities**, seleziona il tuo account sviluppatore
4. `Cmd+R` per eseguire -- Xcode gestisce la firma automaticamente

## Utilizzo

- **Nuova scheda**: `Cmd+N` crea un documento temporaneo, inizia a scrivere immediatamente
- **Apri file**: `Cmd+O` apre file .mint / .txt / .md dal disco
- **Salva**: `Cmd+S` salva la scheda corrente sul disco; documenti temporanei attivano Salva come
- **Salva come**: `Cmd+Shift+S` salva in un formato diverso (.mint / .txt / .md)
- **Alterna rendering**: `Cmd+R` passa tra modifica testo semplice e visualizzazione renderizzata Markdown
- **Trova e sostituisci**: `Cmd+F` per trovare, `Cmd+Option+F` per trovare e sostituire
- **Riordina schede**: Trascina le schede per riorganizzare
- **Chiudi scheda**: `Cmd+W`, chiede di salvare se ci sono modifiche non salvate
- **Controlla aggiornamenti**: Preferenze (`Cmd+,`) ha un pulsante "Controlla aggiornamenti"

## Domande frequenti

**Dove sono memorizzati i documenti temporanei?**

In `~/Library/Application Support/MiniNote/sessions/`. Ogni documento temporaneo è un file separato, più un `session.json` che registra l'ordine delle schede e i metadati.

**Qual è la differenza tra .mint e .txt?**

Sono identici alla base -- entrambi testo semplice. L'unica differenza è che .mint salva aggiuntivamente la posizione del cursore e lo stato di rendering. Puoi convertire liberamente tra loro tramite Salva come senza perdere contenuto.

**Supporta l'evidenziazione della sintassi?**

No. MiniNote è un editor di testo semplice, mantenendolo puro. Il rendering Markdown usa AttributedString di sistema per intestazioni di base, elenchi, grassetto, ecc. Nessuna evidenziazione sintassi del codice.

**In cosa è diverso da TextEdit / CotEditor / BBEdit?**

La filosofia centrale di MiniNote è: persistenza schede (sopravvive ai riavvii) + logica di salvataggio a due livelli (separazione temporanei vs disco). TextEdit non supporta la persistenza delle schede; CotEditor ha più funzionalità ma manca di questo meccanismo; BBEdit è troppo pesante. MiniNote fa solo questa cosa bene.

**Supporta la sincronizzazione cloud?**

No, e non lo farà mai. Tutti i dati sono memorizzati localmente, completamente offline.

## Sviluppo

Stack tecnologico: Swift 6 + SwiftUI + NSTextView (TextKit), zero dipendenze di terze parti.

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# Cmd+B per compilare
```

## Licenza

MIT License
