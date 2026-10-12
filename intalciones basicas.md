# Instalaciones básicas y configuración inicial (iawar)

Este documento resume, de forma breve, lo que se configuró al iniciar el proyecto.

## 1) Actualización de herramientas Tauri con Cargo

Para actualizar `create-tauri-app`:

```bash
cargo install create-tauri-app --locked --force
```

- `--locked`: usa versiones fijadas por el `Cargo.lock` del paquete.
- `--force`: reinstala/actualiza aunque ya esté instalado.

## 2) Identifier correcto para macOS/Xcode

Se definió el identificador en formato reverse-domain:

```text
com.benji.iawar
```

Regla: no usar correo como identifier.

## 3) Tailwind CSS v4 con Bun (Leptos + Tauri)

### Dependencias

```bash
bun add -d tailwindcss @tailwindcss/cli
```

> En Tailwind v4 ya no existe `tailwindcss init -p`.

### Entrada/salida de Tailwind

Se creó el archivo de entrada:

- [tailwind.input.css](/Users/benji/Documents/RUST/iawar/tailwind.input.css)

Contenido:

```css
@import "tailwindcss";
```

Compilación (watch):

```bash
bunx tailwindcss -i ./tailwind.input.css -o ./styles.css --watch
```

Notas:
- `styles.css` queda como archivo generado (salida).
- No usar el mismo archivo como input y output.

### Frontend con utilidades Tailwind

Se reemplazaron clases CSS antiguas por utilidades Tailwind en:

- [src/app.rs](/Users/benji/Documents/RUST/iawar/src/app.rs)

## 4) Barra superior macOS (mantener botones nativos sin franja blanca)

Configuración aplicada en:

- [src-tauri/tauri.conf.json](/Users/benji/Documents/RUST/iawar/src-tauri/tauri.conf.json)

Valores:

```json
"decorations": true,
"titleBarStyle": "Transparent",
"hiddenTitle": true
```

Con esto:
- se mantienen los botones nativos de macOS,
- se oculta el título,
- se evita la configuración `Overlay` que requería drag region manual.

## 5) Git inicial

Se inicializó repositorio local y se creó rama de versionado:

```bash
git init
git checkout -b iawar-1.1.1
```

