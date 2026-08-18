# Uso del SDK en Windows

## Requisitos

- Rust toolchain para Windows (`x86_64-pc-windows-msvc` normalmente)
- Visual Studio Build Tools o MSVC toolchain
- Java 17+ si vas a usar el cliente Java

## Compilación de la librería

Desde el root del proyecto:

```/dev/null/windows-build.ps1#L1-2
cargo build -p zkfp-capi
cargo build --release -p zkfp-capi
```

La librería esperada en Windows será normalmente:

- `target\\debug\\zkfp_capi.dll`
- `target\\release\\zkfp_capi.dll`

## Headers

Los headers públicos están en:

- `crates/zkfp-capi/include/`

## Uso desde C/C++

Debes enlazar contra la DLL/import library generada por Rust según tu toolchain.

## Uso desde Java con JNA

### Copiar la DLL al cliente Java

Desde `java-client\\`:

```/dev/null/windows-copy.ps1#L1-2
cargo build --release -p zkfp-capi --manifest-path ..\\Cargo.toml
Copy-Item ..\\target\\release\\zkfp_capi.dll .\\sdk\\zkfp_capi.dll -Force
```

### Ejecutar

```/dev/null/windows-run.ps1#L1-1
.\\gradlew.bat run
```

## Flujo recomendado en Windows para producción

1. obtener datos desde PostgreSQL u otra fuente remota externa
2. reconstruir snapshot local SQLite con bulk tipado
3. cargar la gallery con `zkfp_gallery_load_from_db()`
4. operar offline contra local

## Notas sobre despliegue

- la DLL debe estar en una ruta visible para JNA
- una opción simple es colocarla en `java-client/sdk/`
- si usas una app propia, puedes distribuir la DLL junto al ejecutable o jar

## USB en Windows

Si `zkfp_init()` falla:

- verifica drivers del dispositivo
- confirma que el escáner sea visible en el administrador de dispositivos
- valida permisos y acceso exclusivo al USB

## Recomendación

En Windows, igual que en Linux, evita usar la ruta JSON como mecanismo principal de rebuild masivo. Usa bulk tipado y carga directa de gallery desde DB local.
