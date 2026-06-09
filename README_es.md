[简体中文](readme.md) | [繁體中文](README_zh-TW.md) | [English](README_en-US.md) | [日本語](README_ja.md) | [한국어](README_ko.md) | [Deutsch](README_de.md) | [Français](README_fr.md) | **Español** | [Português](README_pt-BR.md) | [Italiano](README_it.md) | [Русский](README_ru.md)

# MiniNote

MiniNote es una aplicación de notas de escritorio local-first. Ábrela, escribe de inmediato, guarda todo en tu propia máquina y fija notas en el escritorio cuando las necesites. Sin cuenta, sin sincronización en la nube y sin flujos pesados de base de conocimiento.

## Para qué sirve

- Capturar ideas, comandos, tareas, fragmentos de reuniones o recordatorios de trabajo/juego.
- Mantener texto reutilizable fijado en el escritorio para leerlo o copiarlo rápido.
- Abrir una pequeña ventana de nota sin interrumpir tu tarea actual.
- Organizar borradores locales con categorías simples.
- Escribir Markdown ligero y previsualizar encabezados, listas, citas y bloques de código.

## Funciones principales

- **Biblioteca local de notas**: las notas, categorías y ajustes se guardan localmente; no tienes que elegir primero una ruta de archivo.
- **Notas rápidas**: abre una pequeña ventana desde la bandeja del sistema o con un atajo global; puede aparecer cerca del cursor.
- **Mosaicos de escritorio**: fija una nota en pantalla con colores personalizados y renderizado Markdown opcional.
- **Vista previa Markdown**: útil para texto estructurado cotidiano, sin pretender ser un IDE Markdown completo.
- **Importación y exportación**: admite `.mint`, `.md`, `.markdown` y `.txt`.
- **Protección al sincronizar con el archivo fuente**: los archivos importados pueden conservar un enlace al origen; MiniNote comprueba cambios externos antes de escribir de vuelta.
- **Apariencia configurable**: tema, tamaño de fuente, imagen de fondo, atajos y cierre a la bandeja son ajustables.

## Formatos compatibles

| Formato             | Uso                                                 |
| ------------------- | --------------------------------------------------- |
| `.mint`             | Tipo de documento predeterminado de MiniNote; UTF-8 |
| `.md` / `.markdown` | Documento Markdown                                  |
| `.txt`              | Archivo de texto plano estándar                     |

Todos los archivos compatibles pueden abrirse en un editor de texto normal. MiniNote no escribe metadatos privados en el cuerpo del archivo.

## Instalación y actualización

Las compilaciones oficiales se publican en [GitHub Releases](https://github.com/vivalucas/mininote/releases). A partir de la versión 1.0.0, los archivos oficiales de release se proporcionan solo para Windows y macOS. El soporte de Linux permanece en el código fuente, pero por ahora no se publica ningún paquete oficial para Linux.

### Windows

- Instalador: `mininote-<version>-windows-x64-setup.exe`
- Versión portable: `mininote-<version>-windows-x64.exe`

Usa el instalador para uso habitual. Usa la versión portable si quieres probar MiniNote temporalmente o ejecutarlo desde una carpeta fija.

### macOS

Los usuarios de Apple Silicon deben descargar `mininote-<version>-macos-arm64.dmg`, abrirlo y arrastrar `MiniNote.app` a `Applications`.

La compilación para macOS no está firmada formalmente por ahora. Si macOS indica que la app no puede abrirse, está dañada o proviene de un desarrollador no identificado, primero confirma que el archivo viene de la página Release de este proyecto y luego ejecuta:

```bash
xattr -cr /Applications/MiniNote.app
```

Después abre la aplicación otra vez. Para actualizar, cierra MiniNote, descarga el nuevo DMG y reemplaza la app antigua en `Applications`.

### Linux

MiniNote 1.0.0 no incluye paquetes oficiales para Linux. Si necesitas una compilación de Linux, compílala desde el código fuente; la configuración de empaquetado de Linux se conserva en el repositorio.

## Dónde viven los datos

MiniNote no sube notas ni ofrece sincronización en la nube. Las notas, ajustes y datos de índice se guardan por defecto en la carpeta `MiniNote` dentro del directorio de datos de aplicación del sistema. Si se define `MININOTE_DATA_DIR`, MiniNote usa ese directorio.

## Compilar desde el código fuente

Necesitas Node.js, Rust y las dependencias del sistema requeridas por Tauri.

```bash
npm ci
npm run tauri build
```

Modo de desarrollo:

```bash
npm run tauri dev
```

## Límites

- Sin cuentas ni sincronización en la nube.
- Sin editor de texto enriquecido.
- Sin IDE Markdown completo.
- Sin base de conocimiento compleja, backlinks ni sistema colaborativo.

MiniNote busca mantenerse ligero, rápido, local y cómodo.

## Licencia

MIT License
