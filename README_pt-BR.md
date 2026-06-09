[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | [Español](README_es.md) | **Português** | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

MiniNote é um aplicativo de notas para desktop com foco local. Abra, escreva imediatamente, mantenha tudo na sua própria máquina e fixe notas na área de trabalho quando precisar. Sem conta, sem sincronização em nuvem e sem fluxo pesado de base de conhecimento.

## Para que serve

- Capturar ideias, comandos, tarefas, trechos de reunião ou lembretes de trabalho/jogo.
- Manter texto reutilizável fixado na área de trabalho para leitura e cópia rápidas.
- Abrir uma pequena janela de nota sem interromper a tarefa atual.
- Organizar rascunhos locais com categorias simples.
- Escrever Markdown leve e pré-visualizar títulos, listas, citações e blocos de código.

## Principais recursos

- **Biblioteca local de notas**: notas, categorias e configurações ficam armazenadas localmente; não é preciso escolher um caminho de arquivo primeiro.
- **Notas rápidas**: abra uma pequena janela pela bandeja do sistema ou por um atalho global; ela pode aparecer perto do cursor.
- **Blocos na área de trabalho**: fixe uma nota na tela com cores personalizadas e renderização Markdown opcional.
- **Pré-visualização Markdown**: útil para texto estruturado do dia a dia, sem tentar ser uma IDE Markdown completa.
- **Importação e exportação**: importe um arquivo único ou uma pasta como categoria; compatível com `.mint`, `.md`, `.markdown` e `.txt`.
- **Proteção de sincronização com o arquivo de origem**: arquivos importados e exportados podem manter um vínculo com a origem; o MiniNote verifica mudanças externas antes de gravar de volta.
- **Aparência configurável**: tema, tamanho da fonte, imagem de fundo, atalhos e fechamento para a bandeja podem ser ajustados.

## Formatos compatíveis

| Formato             | Uso                                               |
| ------------------- | ------------------------------------------------- |
| `.mint`             | Tipo de documento padrão do MiniNote; texto UTF-8 |
| `.md` / `.markdown` | Documento Markdown                                |
| `.txt`              | Arquivo de texto simples padrão                   |

Todos os arquivos compatíveis podem ser abertos em um editor de texto comum. O MiniNote não grava metadados privados no corpo do arquivo.

## Instalação e atualização

As compilações oficiais são publicadas no [GitHub Releases](https://github.com/vivalucas/mininote/releases). A partir da versão 1.0.0, os arquivos oficiais de release são fornecidos apenas para Windows e macOS. O suporte a Linux permanece no código-fonte, mas nenhum pacote oficial para Linux é publicado por enquanto.

### Windows

- Instalador: `mininote-<version>-windows-x64-setup.exe`
- Versão portátil: `mininote-<version>-windows-x64.exe`

Use o instalador para uso regular. Use a versão portátil para testar temporariamente ou executar a partir de uma pasta fixa.

### macOS

Usuários de Apple Silicon devem baixar `mininote-<version>-macos-arm64.dmg`, abrir o arquivo e arrastar `MiniNote.app` para `Applications`.

A compilação para macOS não está formalmente assinada no momento. Se o macOS disser que o app não pode ser aberto, está danificado ou vem de um desenvolvedor não identificado, primeiro confirme que o arquivo veio da página Release deste projeto e então execute:

```bash
xattr -cr /Applications/MiniNote.app
```

Depois abra o app novamente. Para atualizar, feche o MiniNote, baixe o novo DMG e substitua o app antigo em `Applications`.

### Linux

O MiniNote 1.0.0 não inclui pacotes oficiais para Linux. Se você precisar de uma versão Linux, compile a partir do código-fonte; a configuração de empacotamento Linux continua no repositório.

## Onde os dados ficam

O MiniNote não envia notas e não oferece sincronização em nuvem. Notas, configurações e dados de índice são armazenados por padrão na pasta `MiniNote` dentro do diretório de dados de aplicativo do sistema. Se `MININOTE_DATA_DIR` estiver definido, o MiniNote usa esse diretório.

## Compilar a partir do código-fonte

Você precisa de Node.js, Rust e das dependências de sistema exigidas pelo Tauri.

```bash
npm ci
npm run tauri build
```

Modo de desenvolvimento:

```bash
npm run tauri dev
```

## Limites

- Sem contas ou sincronização em nuvem.
- Sem editor de texto rico.
- Sem IDE Markdown completa.
- Sem base de conhecimento complexa, backlinks ou sistema colaborativo.

O MiniNote foi feito para permanecer leve, rápido, local e prático.

## Licença

MIT License
