# Uso del SDK en Android

## Estado actual

El SDK actual está centrado en `zkfp-capi` como biblioteca nativa C expuesta desde Rust. La integración con Android es posible, pero requiere trabajo de empaquetado nativo y bindings específicos para Android.

## Enfoque recomendado

Para Android, el camino natural es:

1. compilar la librería nativa para los ABI necesarios
2. empaquetarla en `jniLibs/`
3. exponer bindings vía JNI o JNA compatible con Android (JNI suele ser la opción más robusta)
4. usar la DB local offline y el matching desde la capa nativa

## ABI objetivo

Normalmente al menos:

- `arm64-v8a`
- `armeabi-v7a`
- opcionalmente `x86_64` para emulador

## Librería nativa

En Android la salida no será `libzkfp_capi.so` del host Linux tal cual, sino una compilación cruzada para Android.

El artefacto final debe colocarse en algo como:

```/dev/null/android-tree.txt#L1-4
app/
  src/main/
    jniLibs/arm64-v8a/libzkfp_capi.so
    jniLibs/armeabi-v7a/libzkfp_capi.so
```

## Flujo recomendado en Android

### Snapshot offline

1. descargar o recibir datos remotos desde tu backend
2. reconstruir la DB local SQLite con bulk tipado
3. cargar la gallery con `zkfp_gallery_load_from_db()`
4. operar offline en el dispositivo

## Consideraciones importantes

### USB / scanner

El módulo USB puede requerir adaptación adicional en Android:

- permisos USB Android
- integración con el framework USB host
- posible incompatibilidad del acceso directo usado en escritorio

Por eso, en Android conviene separar claramente:

- DB local offline
- matching
- gestión de plantillas

frente a:

- acceso al escáner USB, que puede requerir una capa específica para Android

## Recomendación práctica

Si vas a priorizar Android, mi recomendación es:

1. usar primero `zkfp-db` + `zkfp-match` + `zkfp-template` en una integración nativa Android
2. validar snapshot local y gallery load
3. después evaluar el soporte real del scanner en Android

## Qué partes del SDK son buenas candidatas en Android

- `zkfp_db_*`
- `zkfp_gallery_*`
- `zkfp_verify_templates`
- `zkfp_extract_from_image_file` si manejas imágenes locales

## Qué parte requiere más validación

- `zkfp_init()` / `zkfp_close()` y la parte USB del hardware en Android real

## Documentación adicional recomendada

Para Android, además de esta guía, conviene mantener un documento de build cruzado por ABI y un ejemplo JNI específico cuando cierres esa integración.
