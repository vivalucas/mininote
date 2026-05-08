# MiniNote

Um editor de texto leve para macOS, inspirado na experiência do Bloco de Notas do Windows 11.

[中文](README.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [Español](README.es.md) | [繁體中文](README.zh-TW.md) | [Italiano](README.it.md) | [Русский](README.ru.md)

---

## Por que criei isso

Mudei do Windows para o Mac e me acostumei com a maioria das coisas, mas sempre senti falta do Bloco de Notas do Windows 11.

Não porque é poderoso -- precisamente porque é simples. Abra e escreva, feche sem perder nada, cole e obtenha texto limpo. Sem formatação, sem rich text, sem sugestões "inteligentes". Apenas um lugar tranquilo para escrever.

Procurei muito no macOS. Ou muito pesado (Obsidian, Typora), muito básico (TextEdit nativo), ou baseado em assinatura. Nada que fosse exatamente o necessário.

Então criei o meu próprio.

## Funcionalidades

- **Persistência de abas**
  - Crie uma nova aba e comece a escrever, o conteúdo é salvo automaticamente em tempo real
  - Desligar, reiniciar, falta de energia -- tudo é restaurado, nada é perdido
  - Fechar a janela preserva silenciosamente todas as abas, sem avisos
  - Fechar uma única aba com mudanças não salvas mostra Salvar / Não Salvar / Cancelar

- **Lógica de salvamento em duas camadas**
  - A camada de sessão registra tudo em tempo real, sobrevive a reinicializações
  - A camada de disco só escreve no arquivo com Cmd+S explícito
  - As duas camadas operam independentemente

- **Colar como texto simples**
  - Colar é texto simples por padrão, sem passos extras
  - Texto copiado de páginas web, WeChat, PDFs é automaticamente limpo da formatação
  - Funciona como "estação de limpeza de formato": cole rich text, copie, obtenha texto limpo

- **Renderização Markdown opcional**
  - Edição de texto simples por padrão, Cmd+R para alternar visualização renderizada
  - Visualização renderizada é somente leitura, volte ao texto simples para continuar editando
  - .mint / .txt / .md têm interruptores de renderização independentes em Preferências

- **Integração com Finder**
  - Clique direito em qualquer pasta do Finder para criar um novo arquivo .mint
  - Suporte nativo ao Quick Look do macOS -- selecione um arquivo e pressione Espaço para visualizar
  - Arquivos .mint abrem com MiniNote por padrão

- **Outros**
  - Barra de status mostra linha/coluna, contagem de caracteres, codificação, fim de linha, modo de renderização
  - Três formatos de arquivo: .mint (padrão), .txt, .md -- converter via Salvar Como
  - Troca de tema: Claro / Escuro / Seguir Sistema
  - Verificar atualizações do GitHub nas Preferências

## Atalhos de teclado

| Função | Atalho |
|----------|----------|
| Novo | `Cmd+N` |
| Abrir | `Cmd+O` |
| Salvar | `Cmd+S` |
| Salvar Como | `Cmd+Shift+S` |
| Fechar Aba | `Cmd+W` |
| Desfazer / Refazer | `Cmd+Z` / `Cmd+Shift+Z` |
| Buscar | `Cmd+F` |
| Buscar e Substituir | `Cmd+Option+F` |
| Alternar Markdown | `Cmd+R` |
| Preferências | `Cmd+,` |

## Formatos suportados

| Formato | Descrição |
|--------|-------------|
| .mint | Formato nativo do MiniNote, padrão para novos arquivos. Texto simples + informações de estado leves (posição do cursor, estado de renderização) |
| .txt | Texto simples padrão, compatível com outros editores |
| .md | Formato Markdown |

Todos três são texto simples no núcleo. Converter entre eles é apenas mudar a extensão.

## Requisitos

- macOS 26 (Tahoe) ou posterior
- Mac com Apple Silicon (chips série M)

## Instalação

**Opção 1: Instalador DMG**

1. Vá para a página de [Releases](../../releases) e baixe o último `MiniNote-[versão].dmg`
2. Abra o DMG e arraste MiniNote para sua pasta de Aplicativos
3. Na primeira inicialização, o macOS pode dizer "o app está danificado" ou "não foi possível verificar o desenvolvedor" -- isso é comportamento normal do Gatekeeper para apps não assinados. Execute isso no Terminal para remover a marca de quarentena:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   Depois clique duplo para iniciar; ou clique direito, Abrir, depois clique em Abrir no diálogo.

**Opção 2: Arquivo ZIP**

1. Vá para a página de [Releases](../../releases) e baixe o último `MiniNote-[versão].zip`, depois descompacte
2. Mova `MiniNote.app` para sua pasta de Aplicativos
3. Execute no Terminal:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**Opção 3: Compilar do código fonte (sem solução de assinatura)**

1. Clone este repositório
2. Abra `MiniNote.xcodeproj` no Xcode
3. Em **Signing & Capabilities**, selecione sua própria conta de desenvolvedor
4. `Cmd+R` para executar -- Xcode cuida da assinatura automaticamente

## Uso

- **Nova Aba**: `Cmd+N` cria um documento temporário, comece a escrever imediatamente
- **Abrir Arquivo**: `Cmd+O` abre arquivos .mint / .txt / .md do disco
- **Salvar**: `Cmd+S` salva a aba atual no disco; documentos temporários acionam Salvar Como
- **Salvar Como**: `Cmd+Shift+S` salva em um formato diferente (.mint / .txt / .md)
- **Alternar Renderização**: `Cmd+R` alterna entre edição de texto simples e visualização renderizada de Markdown
- **Buscar e Substituir**: `Cmd+F` para buscar, `Cmd+Option+F` para buscar e substituir
- **Reordenar Abas**: Arraste as abas para reorganizar
- **Fechar Aba**: `Cmd+W`, pergunta para salvar se houver mudanças não salvas
- **Verificar Atualizações**: Preferências (`Cmd+,`) tem um botão "Verificar Atualizações"

## Perguntas Frequentes

**Onde são armazenados os documentos temporários?**

Em `~/Library/Application Support/MiniNote/sessions/`. Cada documento temporário é um arquivo separado, mais um `session.json` que registra a ordem das abas e metadados.

**Qual a diferença entre .mint e .txt?**

São idênticos no núcleo -- ambos são texto simples. A única diferença é que .mint salva adicionalmente a posição do cursor e estado de renderização. Você pode converter entre eles livremente via Salvar Como sem perder conteúdo.

**Suporta destaque de sintaxe?**

Não. MiniNote é um editor de texto simples, mantendo-o puro. A renderização Markdown usa o AttributedString do sistema para títulos básicos, listas, negrito, etc. Sem destaque de sintaxe de código.

**Como é diferente do TextEdit / CotEditor / BBEdit?**

A filosofia central do MiniNote é: persistência de abas (sobrevive reinícios) + lógica de salvamento em duas camadas (separação entre temporários e disco). TextEdit não suporta persistência de abas; CotEditor tem mais funcionalidades mas carece deste mecanismo; BBEdit é muito pesado. MiniNote faz apenas isso bem feito.

**Suporta sincronização na nuvem?**

Não, e nunca vai suportar. Todos os dados são armazenados localmente, completamente offline.

## Desenvolvimento

Stack tecnológico: Swift 6 + SwiftUI + NSTextView (TextKit), zero dependências de terceiros.

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# Cmd+B para compilar
```

## Licença

MIT License
