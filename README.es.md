# MiniNote

Un editor de texto ligero para macOS, inspirado en la experiencia del Bloc de notas de Windows 11.

[中文](README.md) | [English](README.en.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Deutsch](README.de.md) | [Français](README.fr.md) | [繁體中文](README.zh-TW.md) | [Português](README.pt-BR.md) | [Italiano](README.it.md) | [Русский](README.ru.md)

---

## Por qué lo creé

Cambié de Windows a Mac y me acostumbré a la mayoría de las cosas, pero siempre extrañé el Bloc de notas de Windows 11.

No porque sea poderoso, sino precisamente por ser simple. Abrir y escribir, cerrar sin perder nada, pegar y obtener texto limpio. Sin formato, sin texto enriquecido, sin sugerencias "inteligentes". Solo un lugar tranquilo para escribir.

Busqué mucho en macOS. O demasiado pesado (Obsidian, Typora), demasiado básico (TextEdit integrado), o basado en suscripción. Nada que fuera justo lo necesario.

Así que creé el mío propio.

## Funciones

- **Persistencia de pestañas**
  - Crear una nueva pestaña y empezar a escribir, el contenido se guarda automáticamente en tiempo real
  - Apagar, reiniciar, cortes de energía -- todo se restaura, nada se pierde
  - Cerrar la ventana preserva silenciosamente todas las pestañas, sin mensajes
  - Cerrar una sola pestaña con cambios no guardados muestra Guardar / No guardar / Cancelar

- **Lógica de guardado de dos capas**
  - La capa de sesión registra todo en tiempo real, sobrevive a reinicios
  - La capa de disco solo escribe en el archivo con Cmd+S explícito
  - Ambas capas operan independientemente

- **Pegar como texto plano**
  - Pegar es texto plano por defecto, sin pasos extra
  - Texto copiado de páginas web, WeChat, PDFs se limpia automáticamente del formato
  - Funciona como "estación de limpieza de formato": pega texto con formato, cópialo, obtén texto limpio

- **Renderizado Markdown opcional**
  - Edición de texto plano por defecto, Cmd+R para alternar vista renderizada
  - Vista renderizada es de solo lectura, vuelve a texto plano para seguir editando
  - .mint / .txt / .md tienen interruptores de renderizado independientes en Preferencias

- **Integración con Finder**
  - Clic derecho en cualquier carpeta del Finder para crear un archivo .mint nuevo
  - Soporte nativo de Quick Look de macOS -- selecciona un archivo y presiona Espacio para previsualizar
  - Archivos .mint se abren con MiniNote por defecto

- **Otros**
  - Barra de estado muestra línea/columna, conteo de caracteres, codificación, fin de línea, modo de renderizado
  - Tres formatos de archivo: .mint (por defecto), .txt, .md -- convertir con Guardar como
  - Cambio de tema: Claro / Oscuro / Seguir sistema
  - Buscar actualizaciones de GitHub desde Preferencias

## Disposición

```
+-------------------------------------------+
| Barra de menú                              |
+-------------------------------------------+
| [Sin título]  [notas.mint]  [ideas.md] [+ ] |
+-------------------------------------------+
|                                           |
|              Área de edición              |
|                                           |
+-------------------------------------------+
| Barra de estado (Lín/Col | Caracteres | UTF-8 | LF | Texto) |
+-------------------------------------------+
```

## Atajos de teclado

| Función | Atajo |
|----------|----------|
| Nuevo | `Cmd+N` |
| Abrir | `Cmd+O` |
| Guardar | `Cmd+S` |
| Guardar como | `Cmd+Shift+S` |
| Cerrar pestaña | `Cmd+W` |
| Deshacer / Rehacer | `Cmd+Z` / `Cmd+Shift+Z` |
| Buscar | `Cmd+F` |
| Buscar y reemplazar | `Cmd+Option+F` |
| Alternar Markdown | `Cmd+R` |
| Preferencias | `Cmd+,` |

## Formatos compatibles

| Formato | Descripción |
|--------|-------------|
| .mint | Formato nativo de MiniNote, por defecto para archivos nuevos. Texto plano + información de estado ligera (posición del cursor, estado de renderizado) |
| .txt | Texto plano estándar, compatible con otros editores |
| .md | Formato Markdown |

Los tres son texto plano en su núcleo. Convertir entre ellos es solo cambiar la extensión.

## Requisitos

- macOS 26 (Tahoe) o posterior
- Mac con Apple Silicon (chips serie M)

## Instalación

**Opción 1: Instalador DMG**

1. Ve a la página de [Releases](../../releases) y descarga el último `MiniNote-[versión].dmg`
2. Abre el DMG y arrastra MiniNote a tu carpeta de Aplicaciones
3. En el primer inicio, macOS puede decir "la aplicación está dañada" o "no se puede verificar el desarrollador" -- esto es comportamiento normal de Gatekeeper para aplicaciones sin firmar. Ejecuta esto en Terminal para quitar la marca de cuarentena:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```
   Luego doble clic para iniciar; o clic derecho, Abrir, luego clic en Abrir en el diálogo.

**Opción 2: Archivo ZIP**

1. Ve a la página de [Releases](../../releases) y descarga el último `MiniNote-[versión].zip`, luego descomprime
2. Mueve `MiniNote.app` a tu carpeta de Aplicaciones
3. Ejecuta en Terminal:
   ```bash
   xattr -cr /Applications/MiniNote.app
   ```

**Opción 3: Compilar desde el código fuente (sin solución de firmado)**

1. Clona este repositorio
2. Abre `MiniNote.xcodeproj` en Xcode
3. En **Signing & Capabilities**, selecciona tu propia cuenta de desarrollador
4. `Cmd+R` para ejecutar -- Xcode maneja el firmado automáticamente

## Uso

- **Nueva pestaña**: `Cmd+N` crea un documento temporal, empieza a escribir inmediatamente
- **Abrir archivo**: `Cmd+O` abre archivos .mint / .txt / .md del disco
- **Guardar**: `Cmd+S` guarda la pestaña actual al disco; documentos temporales activan Guardar como
- **Guardar como**: `Cmd+Shift+S` guarda en un formato diferente (.mint / .txt / .md)
- **Alternar renderizado**: `Cmd+R` cambia entre edición de texto plano y vista renderizada de Markdown
- **Buscar y reemplazar**: `Cmd+F` para buscar, `Cmd+Option+F` para buscar y reemplazar
- **Reordenar pestañas**: Arrastra las pestañas para reorganizar
- **Cerrar pestaña**: `Cmd+W`, pregunta para guardar si hay cambios no guardados
- **Buscar actualizaciones**: Preferencias (`Cmd+,`) tiene un botón "Buscar actualizaciones"

## Preguntas frecuentes

**¿Dónde se guardan los documentos temporales?**

En `~/Library/Application Support/MiniNote/sessions/`. Cada documento temporal es un archivo separado, más un `session.json` que registra el orden de pestañas y metadatos.

**¿Cuál es la diferencia entre .mint y .txt?**

Son idénticos en su núcleo -- ambos son texto plano. La única diferencia es que .mint guarda adicionalmente la posición del cursor y el estado de renderizado. Puedes convertir entre ellos libremente con Guardar como sin perder contenido.

**¿Soporta resaltado de sintaxis?**

No. MiniNote es un editor de texto plano, manteniéndolo puro. El renderizado Markdown usa AttributedString del sistema para encabezados básicos, listas, negritas, etc. Sin resaltado de sintaxis de código.

**¿En qué se diferencia de TextEdit / CotEditor / BBEdit?**

La filosofía central de MiniNote es: persistencia de pestañas (sobrevive reinicios) + lógica de guardado de dos capas (separación entre temporales y disco). TextEdit no soporta persistencia de pestañas; CotEditor tiene más funciones pero carece de este mecanismo; BBEdit es demasiado pesado. MiniNote hace solo esto bien.

**¿Soporta sincronización en la nube?**

No, y nunca lo hará. Todos los datos se guardan localmente, completamente offline.

## Desarrollo

Stack tecnológico: Swift 6 + SwiftUI + NSTextView (TextKit), cero dependencias de terceros.

```bash
git clone https://github.com/vivalucas/mininote.git
open MiniNote.xcodeproj
# Cmd+B para compilar
```

## Licencia

MIT License
