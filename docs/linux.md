# Uso del SDK en Linux

## Requisitos

- Rust toolchain
- `cargo`
- `gcc` / toolchain C
- `libusb` si vas a usar el módulo USB
- Java 17+ si vas a usar el cliente Java

## Compilación de la librería

Desde el root del proyecto:

```/dev/null/linux-build.sh#L1-2
cargo build -p zkfp-capi
cargo build --release -p zkfp-capi
```

Salida esperada:

- `target/debug/libzkfp_capi.so`
- `target/release/libzkfp_capi.so`

## Headers

Los headers públicos están en:

- `crates/zkfp-capi/include/`

Header umbrella:

- `crates/zkfp-capi/include/zkfp.h`

## Uso desde C/C++

### Compilar un ejemplo

```/dev/null/linux-cc.sh#L1-1
gcc main.c -Icrates/zkfp-capi/include -Ltarget/release -lzkfp_capi -o app
```

### Ejecutar con `LD_LIBRARY_PATH`

```/dev/null/linux-run.sh#L1-1
LD_LIBRARY_PATH=target/release ./app
```

## Uso desde Java

El cliente Java actual usa JNA.

### Copiar la librería al cliente Java

Desde `java-client/`:

```/dev/null/linux-java-copy.sh#L1-2
cargo build --release -p zkfp-capi --manifest-path ../Cargo.toml
cp ../target/release/libzkfp_capi.so sdk/libzkfp_capi.so
```

### Ejecutar el cliente

```/dev/null/linux-java-run.sh#L1-1
./gradlew run
```

## Flujo recomendado en Linux para producción

1. leer datos remotos desde PostgreSQL
2. reconstruir la DB local con bulk tipado:
   - `zkfp_db_bulk_begin()`
   - `zkfp_db_bulk_row_begin()`
   - `zkfp_db_bulk_row_add_value()`
   - `zkfp_db_bulk_row_insert()`
   - `zkfp_db_bulk_commit()`
3. cargar la gallery desde la DB local:
   - `zkfp_gallery_load_from_db()`
4. operar identificación offline

## Scanner USB en Linux

Si `zkfp_init()` falla por permisos o por dispositivo no encontrado:

- verifica que el escáner esté conectado
- comprueba permisos USB / reglas `udev`
- revisa que el VID/PID esperado corresponda a tu hardware

## Notas

- `zkfp-capi` es un `cdylib`
- el SDK usa estado global interno
- la DB local y la gallery son globales dentro del proceso
- para cargas grandes, evita la ruta CRUD JSON normal y usa bulk tipado
