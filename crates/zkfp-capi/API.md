# zkfp-capi — Referencia oficial del SDK C

`zkfp-capi` expone un SDK C modular sobre los componentes base del proyecto:

- `zkfp-usb`
- `zkfp-image`
- `zkfp-template`
- `zkfp-match`
- `zkfp-db`
- `nbis-rs`

La biblioteca compartida generada es `libzkfp_capi.so`.

---

## Organización del SDK

El SDK está separado por headers y dominios funcionales.

### Headers disponibles

- `zkfp_common.h` — tipos compartidos, errores y liberación de memoria
- `zkfp_usb.h` — ciclo de vida del escáner
- `zkfp_image.h` — captura de imagen y configuración de mejora
- `zkfp_template.h` — extracción de plantillas ISO desde scanner o archivos
- `zkfp_match.h` — comparación 1:1 e identificación en galería en memoria
- `zkfp_db.h` — persistencia genérica, bulk load y consultas JSON sobre `zkfp-db`
- `zkfp_sync.h` — scheduler y sync PostgreSQL → local
- `zkfp.h` — umbrella header que incluye todos los anteriores

Si no necesitas todo el SDK, puedes incluir solo el header del módulo que vayas a usar.

---

## Modelo de estado global

El SDK mantiene estados globales internos independientes por subsistema:

- **scanner global** para el dispositivo USB
- **db global** para la instancia abierta de `zkfp-db`
- **matcher global** para operaciones de matching
- **gallery global** para identificación en memoria
- **extractor global** para NBIS
- **enhance config global** para procesamiento de imagen
- **last error thread-local** para reporte de errores

Esto significa:

- solo puede haber **un escáner abierto a la vez**
- solo puede haber **una instancia DB abierta a la vez** dentro de esta API C
- las llamadas del SDK no usan handles explícitos; operan sobre esos estados internos

---

## Convenciones generales

### Códigos de retorno

La mayoría de funciones usan estas reglas:

- `1` = éxito
- `0` = fallo
- algunas funciones devuelven `int64_t` o `char*`

### Errores

En caso de fallo, consulta:

```/dev/null/api.md#L1-1
zkfp_get_last_error()
```

El puntero devuelto:

- **no debe liberarse**
- es válido hasta la siguiente llamada a `zkfp_get_last_error()` en el mismo hilo

### Ownership de memoria

#### Cadenas devueltas por el SDK

Toda función que devuelve `char*` con memoria reservada por el SDK debe liberarse con:

```/dev/null/api.md#L1-1
zkfp_free_string(...)
```

#### Plantillas devueltas por el SDK

Toda `ZkfpTemplate` llenada por el SDK debe liberarse con:

```/dev/null/api.md#L1-1
zkfp_free_template(...)
```

### JSON en el módulo DB

Las funciones `*_json` del módulo DB devuelven JSON UTF-8 serializado como `char*`.

- el llamador debe liberar con `zkfp_free_string()`
- si una fila no existe, `zkfp_db_get_row_json()` devuelve el JSON `null`
- los `Blob` se serializan como **array de bytes**

---

## Tipos compartidos

### `ZkfpEnhanceConfig`

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_common.h#L9-16
typedef struct ZkfpEnhanceConfig {
    int apply_enhancement;
    int method;
    unsigned char bg_intensity;
    int invert;
    int flip_vertical;
    unsigned int padding;
} ZkfpEnhanceConfig;
```

#### Valores

- `apply_enhancement`: `0` desactiva mejora, `1` la activa
- `method`:
  - `0` = `Stretch`
  - `1` = `Darken`
- `bg_intensity`: 0–255
- `invert`: `0` no, `1` sí
- `flip_vertical`: `0` no, `1` sí
- `padding`: borde blanco en píxeles

### `ZkfpTemplate`

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_common.h#L18-22
typedef struct ZkfpTemplate {
    unsigned char* data;
    uintptr_t size;
    uint32_t quality;
} ZkfpTemplate;
```

Contiene una plantilla ISO/IEC 19794-2:2005.

### `ZkfpIdentifyVerifyResult`

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_common.h#L24-30
typedef struct ZkfpIdentifyVerifyResult {
    uint32_t user_id;
    int identify_score;
    int verify_score;
    int identify_match;
    int verify_match;
} ZkfpIdentifyVerifyResult;
```

---

## Módulo `zkfp_usb.h`

Header:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_usb.h#L1-16
#ifndef ZKFP_USB_H
#define ZKFP_USB_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_init(void);
void zkfp_close(void);

#ifdef __cplusplus
}
#endif

#endif
```

### Funciones

| Función | Descripción |
|---|---|
| `zkfp_init()` | Abre el ZK9500, ejecuta `init()` y enciende LED verde. |
| `zkfp_close()` | Cierra el dispositivo y apaga el LED. |

### Ejemplo

```/dev/null/example.c#L1-9
#include "zkfp_usb.h"

int main(void) {
    if (!zkfp_init()) return 1;
    zkfp_close();
    return 0;
}
```

---

## Módulo `zkfp_image.h`

Header:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_image.h#L1-25
#ifndef ZKFP_IMAGE_H
#define ZKFP_IMAGE_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

char* zkfp_capture_image_base64(const char* format);
int zkfp_image_file_to_base64(const char* path, const char* format, char** out_base64);
int zkfp_set_enhance_config(const ZkfpEnhanceConfig* config);
int zkfp_get_enhance_config(ZkfpEnhanceConfig* config);
int zkfp_set_contrast_method(int method);
int zkfp_set_invert(int invert);
int zkfp_set_flip_vertical(int flip);
int zkfp_set_bg_intensity(unsigned char intensity);
int zkfp_set_padding(unsigned int padding);
int zkfp_set_enhancement_enabled(int enabled);

#ifdef __cplusplus
}
#endif

#endif
```

### Captura y exportación

| Función | Descripción |
|---|---|
| `zkfp_capture_image_base64(format)` | Captura desde el scanner y devuelve imagen codificada en Base64. |
| `zkfp_image_file_to_base64(path, format, out_base64)` | Lee archivo de imagen, aplica mejora y devuelve Base64. |

### Formatos soportados

- `png`
- `bmp`
- `wsq`

### Configuración de mejora

| Función | Descripción |
|---|---|
| `zkfp_set_enhance_config` | Aplica toda la configuración de mejora de una vez. |
| `zkfp_get_enhance_config` | Lee la configuración actual. |
| `zkfp_set_contrast_method` | `0=Stretch`, `1=Darken`. |
| `zkfp_set_invert` | Invierte polaridad. |
| `zkfp_set_flip_vertical` | Voltea verticalmente. |
| `zkfp_set_bg_intensity` | Ajusta intensidad máxima del fondo. |
| `zkfp_set_padding` | Añade padding blanco. |
| `zkfp_set_enhancement_enabled` | Activa o desactiva mejora. |

### Pipeline de mejora

Cuando `apply_enhancement = 1`:

1. inversión opcional
2. contraste (`Stretch` o `Darken`)
3. padding opcional
4. flip vertical opcional

### Ejemplo

```/dev/null/example.c#L1-16
#include "zkfp_usb.h"
#include "zkfp_image.h"

int main(void) {
    if (!zkfp_init()) return 1;

    zkfp_set_contrast_method(1);
    zkfp_set_bg_intensity(240);
    zkfp_set_invert(1);

    char* b64 = zkfp_capture_image_base64("png");
    if (b64) zkfp_free_string(b64);

    zkfp_close();
    return 0;
}
```

---

## Módulo `zkfp_template.h`

Header:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_template.h#L1-20
#ifndef ZKFP_TEMPLATE_H
#define ZKFP_TEMPLATE_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_extract_template(const unsigned char* bmp_data, uintptr_t bmp_size, ZkfpTemplate* out_template);
int zkfp_capture_full(ZkfpTemplate* out_template, char** out_base64_png);
int zkfp_capture_and_extract_template(ZkfpTemplate* out_template);
int zkfp_extract_from_image_file(const char* path, ZkfpTemplate* out_template, char** out_base64_png);
int zkfp_extract_from_bmp_file(const char* path, ZkfpTemplate* out_template, char** out_base64_png);

#ifdef __cplusplus
}
#endif

#endif
```

### Funciones

| Función | Descripción |
|---|---|
| `zkfp_extract_template` | Extrae plantilla desde bytes BMP en memoria. |
| `zkfp_capture_full` | Captura desde el scanner, extrae plantilla y opcionalmente PNG Base64. |
| `zkfp_capture_and_extract_template` | Variante legacy que solo devuelve plantilla. |
| `zkfp_extract_from_image_file` | Extrae plantilla desde imagen en disco. |
| `zkfp_extract_from_bmp_file` | Alias backward-compatible. |

### Ejemplo

```/dev/null/example.c#L1-17
#include "zkfp_usb.h"
#include "zkfp_template.h"

int main(void) {
    ZkfpTemplate tmpl = {0};
    char* preview = NULL;

    if (!zkfp_init()) return 1;
    if (!zkfp_capture_full(&tmpl, &preview)) return 1;

    zkfp_free_template(&tmpl);
    if (preview) zkfp_free_string(preview);
    zkfp_close();
    return 0;
}
```

---

## Módulo `zkfp_match.h`

Header:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_match.h#L1-21
#ifndef ZKFP_MATCH_H
#define ZKFP_MATCH_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_verify_templates(const unsigned char* tmpl1_data, uintptr_t tmpl1_size, const unsigned char* tmpl2_data, uintptr_t tmpl2_size);
void zkfp_gallery_clear(void);
int zkfp_gallery_add(uint32_t user_id, const unsigned char* tmpl_data, uintptr_t tmpl_size);
int zkfp_gallery_load_from_db(const char* table_name, const char* user_id_column, const char* template_column);
void zkfp_gallery_remove(uint32_t user_id);
int zkfp_gallery_identify(const unsigned char* probe_data, uintptr_t probe_size, uint32_t* out_id, int* out_score);
int zkfp_gallery_identify_with_verification(const unsigned char* probe_data, uintptr_t probe_size, ZkfpIdentifyVerifyResult* out_result);

#ifdef __cplusplus
}
#endif

#endif
```

### Funciones

| Función | Descripción |
|---|---|
| `zkfp_verify_templates` | Comparación 1:1 entre dos plantillas ISO. |
| `zkfp_gallery_clear` | Limpia la galería en memoria. |
| `zkfp_gallery_add` | Inserta una plantilla en la galería. |
| `zkfp_gallery_load_from_db` | Carga la galería directamente desde la DB local sin pasar por JSON en el cliente. |
| `zkfp_gallery_remove` | Elimina todas las entradas de un `user_id`. |
| `zkfp_gallery_identify` | Hace identificación 1:N contra la galería. |
| `zkfp_gallery_identify_with_verification` | Identifica y luego verifica contra el mejor candidato. |

### Ejemplo

```/dev/null/example.c#L1-20
#include "zkfp_match.h"

int main(void) {
    unsigned char probe[] = {0};
    unsigned char candidate[] = {0};
    int score = zkfp_verify_templates(probe, sizeof(probe), candidate, sizeof(candidate));
    (void)score;
    return 0;
}
```

---

## Módulo `zkfp_db.h`

Header:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_db.h#L1-74
#ifndef ZKFP_DB_H
#define ZKFP_DB_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_db_open(const char* db_path);
void zkfp_db_close(void);
int zkfp_db_register_table(const char* table_name);
int zkfp_db_create_fingerprint_schema(
    const char* users_table,
    const char* templates_table,
    const char* user_name_column,
    const char* template_user_id_column,
    const char* template_finger_column,
    const char* template_data_column,
    const char* template_quality_column
);
int zkfp_db_add_column_and_create(
    const char* table_name,
    const char* column_name,
    int data_type,
    unsigned int constraint_flags,
    const char* foreign_table,
    const char* foreign_column
);
int zkfp_db_insert_kv(
    const char* table_name,
    const char* column_name,
    int value_type,
    const char* value,
    int64_t* out_id
);
int zkfp_db_insert_json(const char* table_name, const char* json_object, int64_t* out_id);
int zkfp_db_update_kv(
    const char* table_name,
    int64_t row_id,
    const char* column_name,
    int value_type,
    const char* value
);
int zkfp_db_update_json(const char* table_name, int64_t row_id, const char* json_object);
int zkfp_db_delete_row(const char* table_name, int64_t row_id);
char* zkfp_db_get_row_json(const char* table_name, int64_t row_id);
char* zkfp_db_list_rows_json(const char* table_name);
char* zkfp_db_query_eq_json(
    const char* table_name,
    const char* column_name,
    int value_type,
    const char* value
);
char* zkfp_db_query_like_json(
    const char* table_name,
    const char* column_name,
    const char* pattern
);
char* zkfp_db_list_tables_json(void);
char* zkfp_db_get_schema_json(const char* table_name);
int64_t zkfp_db_count(const char* table_name);
int zkfp_db_delete_all_rows(const char* table_name);
int zkfp_db_bulk_begin(void);
int zkfp_db_bulk_insert_json(const char* table_name, const char* json_object, int64_t* out_id);
int zkfp_db_bulk_row_begin(void);
int zkfp_db_bulk_row_add_value(
    const char* column_name,
    int value_type,
    const char* value,
    const unsigned char* blob_data,
    uintptr_t blob_size
);
int zkfp_db_bulk_row_insert(const char* table_name, int64_t* out_id);
int zkfp_db_bulk_commit(void);
int zkfp_db_bulk_rollback(void);

#ifdef __cplusplus
}
#endif

#endif
```

### Apertura y cierre

| Función | Descripción |
|---|---|
| `zkfp_db_open` | Abre la instancia global DB. |
| `zkfp_db_close` | Libera la instancia global DB. |

### Gestión de esquema

| Función | Descripción |
|---|---|
| `zkfp_db_register_table` | Crea una tabla sin columnas de negocio explícitas. |
| `zkfp_db_create_fingerprint_schema` | Crea un esquema mínimo configurable para usuarios y plantillas. |
| `zkfp_db_add_column_and_create` | Crea una tabla con una columna configurable y restricciones. |
| `zkfp_db_list_tables_json` | Devuelve la lista de tablas registradas como JSON. |
| `zkfp_db_get_schema_json` | Devuelve el esquema de una tabla como JSON. |

### CRUD

| Función | Descripción |
|---|---|
| `zkfp_db_insert_kv` | Inserta una sola columna/valor. |
| `zkfp_db_insert_json` | Inserta un objeto JSON completo. |
| `zkfp_db_update_kv` | Actualiza una sola columna/valor por `row_id`. |
| `zkfp_db_update_json` | Actualiza múltiples columnas usando un objeto JSON. |
| `zkfp_db_delete_row` | Elimina una fila por `row_id`. |
| `zkfp_db_delete_all_rows` | Elimina todas las filas de una tabla. |

### Bulk load genérico

Estas funciones están pensadas para reconstruir snapshots locales offline de forma masiva.

Hay dos niveles de API bulk:

1. **bulk por fila JSON**: útil para interoperabilidad rápida
2. **bulk tipado por columna**: recomendado para producción cuando ya conoces el esquema y quieres menos overhead en cliente

| Función | Descripción |
|---|---|
| `zkfp_db_bulk_begin` | Inicia una transacción SQLite masiva. |
| `zkfp_db_bulk_insert_json` | Inserta una fila JSON dentro de la transacción bulk. |
| `zkfp_db_bulk_row_begin` | Inicia un buffer de fila tipada para inserción bulk. |
| `zkfp_db_bulk_row_add_value` | Añade una columna tipada al buffer de fila actual. |
| `zkfp_db_bulk_row_insert` | Inserta la fila tipada actual dentro de la transacción bulk. |
| `zkfp_db_bulk_commit` | Confirma la carga masiva. |
| `zkfp_db_bulk_rollback` | Revierte la carga masiva si algo falla. |

#### Notas de rendimiento

- `zkfp_db_bulk_insert_json` evita la ruta normal de CRUD usada por `zkfp_db_insert_json`
- `zkfp_db_bulk_row_begin` + `zkfp_db_bulk_row_add_value` + `zkfp_db_bulk_row_insert` ofrecen una vía más alineada con el esquema tipado del crate
- la ruta tipada permite declarar valores columna por columna sin construir una fila JSON completa en el cliente
- en la ruta tipada, los `Blob` deben enviarse por `blob_data` + `blob_size`
- para `value_type = 3 (Blob)`, el parámetro `value` se ignora
- está orientada a cargas completas tipo snapshot
- evita la cola de sync interna usada por inserciones CRUD normales
- debe usarse entre `zkfp_db_bulk_begin()` y `zkfp_db_bulk_commit()`
- para rebuild masivo, el SDK aplica pragmas SQLite de carga rápida durante la transacción bulk

### Lectura y consultas JSON

| Función | Descripción |
|---|---|
| `zkfp_db_get_row_json` | Devuelve una fila como objeto JSON o `null`. |
| `zkfp_db_list_rows_json` | Devuelve todas las filas como array JSON. |
| `zkfp_db_query_eq_json` | Consulta por igualdad y devuelve array JSON. |
| `zkfp_db_query_like_json` | Consulta usando `LIKE` y devuelve array JSON. |
| `zkfp_db_count` | Cuenta registros de una tabla. |

### Tipos del DB SDK

#### `data_type`

- `0` = `Integer`
- `1` = `Real`
- `2` = `Text`
- `3` = `Blob`
- `4` = `Boolean`
- `5` = `Timestamp`

#### `constraint_flags`

Bitmask:

- `0x01` = `PrimaryKey`
- `0x02` = `NotNull`
- `0x04` = `Unique`
- `0x08` = `AutoIncrement`
- `0x10` = `ForeignKey`

#### `value_type`

- `0` = `Integer`
- `1` = `Real`
- `2` = `Text`
- `3` = `Boolean`
- `4` = `Null`
- `5` = `JSON`

### Ejemplo de CRUD JSON

```/dev/null/example.c#L1-30
#include "zkfp_db.h"
#include "zkfp_common.h"

int main(void) {
    int64_t row_id = 0;
    char* rows = NULL;

    if (!zkfp_db_open("fingerprint.db")) return 1;

    zkfp_db_create_fingerprint_schema(
        "users",
        "templates",
        "name",
        "user_id",
        "finger",
        "template_data",
        "quality"
    );

    if (!zkfp_db_insert_json("users", "{\"name\":\"Alice\"}", &row_id)) return 1;
    if (!zkfp_db_update_kv("users", row_id, "name", 2, "Alice Smith")) return 1;

    rows = zkfp_db_list_rows_json("users");
    if (rows) zkfp_free_string(rows);

    zkfp_db_close();
    return 0;
}
```

### Ejemplo de bulk tipado por columnas

```/dev/null/example_bulk_typed.c#L1-38
#include <stdint.h>
#include "zkfp_db.h"
#include "zkfp_common.h"

int main(void) {
    int64_t row_id = 0;
    unsigned char tmpl[] = {1, 2, 3, 4, 5};

    if (!zkfp_db_open("fingerprint.db")) return 1;
    if (!zkfp_db_create_fingerprint_schema("users", "templates", "name", "user_id", "finger", "template_data", "quality")) return 1;

    if (!zkfp_db_bulk_begin()) return 1;

    if (!zkfp_db_bulk_row_begin()) return 1;
    if (!zkfp_db_bulk_row_add_value("id", 0, "1", NULL, 0)) return 1;
    if (!zkfp_db_bulk_row_add_value("name", 2, "Alice", NULL, 0)) return 1;
    if (!zkfp_db_bulk_row_insert("users", &row_id)) return 1;

    if (!zkfp_db_bulk_row_begin()) return 1;
    if (!zkfp_db_bulk_row_add_value("id", 0, "1", NULL, 0)) return 1;
    if (!zkfp_db_bulk_row_add_value("user_id", 0, "1", NULL, 0)) return 1;
    if (!zkfp_db_bulk_row_add_value("finger", 2, "right_index", NULL, 0)) return 1;
    if (!zkfp_db_bulk_row_add_value("template_data", 3, NULL, tmpl, sizeof(tmpl))) return 1;
    if (!zkfp_db_bulk_row_add_value("quality", 0, "50", NULL, 0)) return 1;
    if (!zkfp_db_bulk_row_insert("templates", &row_id)) return 1;

    if (!zkfp_db_bulk_commit()) return 1;

    zkfp_db_close();
    return 0;
}
```

### Ejemplo de carga de gallery desde DB local

```/dev/null/example_gallery_load.c#L1-15
#include "zkfp_match.h"
#include "zkfp_db.h"

int main(void) {
    if (!zkfp_db_open("fingerprint.db")) return 1;

    int loaded = zkfp_gallery_load_from_db("templates", "user_id", "template_data");
    if (loaded <= 0) return 1;

    zkfp_db_close();
    return 0;
}
```

---

## Módulo `zkfp_sync.h`

Header:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp_sync.h#L1-35
#ifndef ZKFP_SYNC_H
#define ZKFP_SYNC_H

#include "zkfp_common.h"

#ifdef __cplusplus
extern "C" {
#endif

int zkfp_sync_config_reset(void);
int zkfp_sync_set_postgres_url(const char* postgres_url);
int zkfp_sync_set_interval_seconds(unsigned int seconds);
int zkfp_sync_set_daily_time(unsigned char hour, unsigned char minute);
int zkfp_sync_set_weekly_time(const char* weekdays_csv, unsigned char hour, unsigned char minute);
int zkfp_sync_set_cron(const char* cron_expression);
int zkfp_sync_set_manual(void);
int zkfp_sync_add_mapping(
    const char* postgres_query,
    const char* local_table,
    const char* mappings_json,
    int strategy,
    const char* unique_keys_csv
);
int zkfp_sync_apply_config(void);
int zkfp_sync_start(void);
int zkfp_sync_stop(void);
int zkfp_sync_run_now(void);
int zkfp_sync_is_running(void);
char* zkfp_sync_get_last_sync_at(void);
```

### Propósito

Este módulo expone la sincronización **unidireccional PostgreSQL → SQLite local** implementada en `zkfp-db`.

El flujo esperado es:

1. abrir la DB local con `zkfp_db_open`
2. definir el esquema local destino
3. resetear/configurar sync
4. añadir uno o más mappings `query PostgreSQL -> tabla local`
5. aplicar la configuración
6. iniciar daemon programado o lanzar sincronización manual

### Reglas importantes

- la sincronización solo copia datos **desde PostgreSQL hacia la DB local**
- no empuja datos locales hacia PostgreSQL
- `zkfp_sync_start()` requiere que la DB local global ya esté abierta
- `zkfp_sync_apply_config()` crea el engine interno a partir del `SyncConfig` acumulado
- `zkfp_sync_get_last_sync_at()` devuelve una cadena RFC3339 y debe liberarse con `zkfp_free_string()`
- `zkfp_sync_is_running()` indica si existe engine configurado; no distingue con precisión fina entre “configurado” y “daemon activo”

### Configuración general

| Función | Descripción |
|---|---|
| `zkfp_sync_config_reset` | Restablece la configuración a valores por defecto. |
| `zkfp_sync_set_postgres_url` | Define la cadena de conexión PostgreSQL. |
| `zkfp_sync_apply_config` | Materializa la configuración actual en un `SyncEngine`. |

### Programación

| Función | Descripción |
|---|---|
| `zkfp_sync_set_interval_seconds` | Ejecuta sync cada `N` segundos. |
| `zkfp_sync_set_daily_time` | Ejecuta sync una vez al día en `hour:minute` UTC. |
| `zkfp_sync_set_weekly_time` | Ejecuta sync ciertos días de la semana en `hour:minute` UTC. |
| `zkfp_sync_set_cron` | Usa una expresión cron válida. |
| `zkfp_sync_set_manual` | Desactiva ejecución automática; solo manual. |

### Mappings y ejecución

| Función | Descripción |
|---|---|
| `zkfp_sync_add_mapping` | Registra una consulta PostgreSQL y cómo escribir sus columnas en una tabla local. |
| `zkfp_sync_start` | Arranca el daemon programado según el schedule configurado. |
| `zkfp_sync_stop` | Detiene el daemon programado. |
| `zkfp_sync_run_now` | Ejecuta una sincronización inmediata. |
| `zkfp_sync_get_last_sync_at` | Devuelve el timestamp de la última sync completada. |

### Estrategias de escritura

`strategy` en `zkfp_sync_add_mapping` usa estos valores:

- `0` = `Replace`
- `1` = `Append`
- `2` = `Upsert`

#### Semántica práctica

- `Replace`: borra toda la tabla destino antes de insertar el resultado de la query
- `Append`: inserta registros sin borrar primero
- `Upsert`: hace escritura tipo `REPLACE INTO` en SQLite

### Parámetros de `zkfp_sync_add_mapping`

#### `postgres_query`

Consulta SQL que se ejecuta en PostgreSQL.

Ejemplo:

```/dev/null/example.sql#L1-3
SELECT id, full_name, active
FROM remote_users
WHERE active = true
```

#### `local_table`

Nombre de la tabla SQLite local destino. Debe existir previamente o la operación fallará.

#### `mappings_json`

Objeto JSON donde:

- la **clave** es la columna devuelta por PostgreSQL
- el **valor** es la columna destino en SQLite

Ejemplo:

```/dev/null/mapping.json#L1-1
{"id":"id","full_name":"name","active":"enabled"}
```

#### `unique_keys_csv`

Actualmente se acepta como parte del contrato de API, pero la implementación actual de `Upsert` usa `REPLACE INTO` sobre la tabla y depende de que exista una clave primaria o restricción única en SQLite.

### Ejemplo: sync manual

```/dev/null/example_sync_manual.c#L1-34
#include <stdio.h>
#include "zkfp.h"

int main(void) {
    if (!zkfp_db_open("fingerprint.db")) return 1;

    if (!zkfp_db_add_column_and_create("users_sync", "id", 0, 0x01, NULL, NULL)) return 1;
    if (!zkfp_db_add_column_and_create("users_sync", "name", 2, 0, NULL, NULL)) return 1;

    if (!zkfp_sync_config_reset()) return 1;
    if (!zkfp_sync_set_postgres_url("host=127.0.0.1 port=5432 user=postgres password=secret dbname=zkfp")) return 1;
    if (!zkfp_sync_set_manual()) return 1;

    if (!zkfp_sync_add_mapping(
        "SELECT id, full_name FROM users",
        "users_sync",
        "{\"id\":\"id\",\"full_name\":\"name\"}",
        2,
        "id"
    )) return 1;

    if (!zkfp_sync_apply_config()) return 1;
    if (!zkfp_sync_run_now()) {
        fprintf(stderr, "%s\n", zkfp_get_last_error());
        return 1;
    }

    zkfp_db_close();
    return 0;
}
```

### Ejemplo: sync diario

```/dev/null/example_sync_daily.c#L1-27
#include "zkfp.h"

int main(void) {
    if (!zkfp_db_open("fingerprint.db")) return 1;

    if (!zkfp_sync_config_reset()) return 1;
    if (!zkfp_sync_set_postgres_url("host=127.0.0.1 port=5432 user=postgres password=secret dbname=zkfp")) return 1;
    if (!zkfp_sync_set_daily_time(2, 30)) return 1;

    if (!zkfp_sync_add_mapping(
        "SELECT id, template_data, quality FROM templates",
        "templates",
        "{\"id\":\"id\",\"template_data\":\"template_data\",\"quality\":\"quality\"}",
        2,
        "id"
    )) return 1;

    if (!zkfp_sync_apply_config()) return 1;
    if (!zkfp_sync_start()) return 1;

    /* tu proceso principal seguiría vivo aquí */

    if (!zkfp_sync_stop()) return 1;
    zkfp_db_close();
    return 0;
}
```

### Integración orientativa desde Java con JNA

En el cliente Java actual, la reconstrucción del snapshot local ya no necesita depender del sync complejo por mappings. Para el caso de snapshot offline completo, el flujo recomendado es:

1. leer datos desde PostgreSQL
2. borrar/recrear la DB local
3. usar `zkfp_db_bulk_begin()`
4. insertar todas las filas con `zkfp_db_bulk_row_begin()` + `zkfp_db_bulk_row_add_value()` + `zkfp_db_bulk_row_insert()`
5. confirmar con `zkfp_db_bulk_commit()`
6. cargar la galería RAM directamente desde la DB local con `zkfp_gallery_load_from_db()`

El cliente Java actual usa JNA, así que el consumo natural de esta API es exponer estas firmas en una interfaz como `ZkfpLibrary`.

#### Modelo recomendado para el cliente Java

Para el caso de uso de cliente operativo, el flujo recomendado es:

1. PostgreSQL actúa como fuente remota de actualización
2. el SDK sincroniza periódicamente `PostgreSQL -> SQLite local`
3. la app Java consulta la DB local del SDK y carga galerías directamente desde SQLite local a la memoria nativa
4. la identificación y lectura operan contra local

Si se mantiene una función de enrolamiento directo a PostgreSQL para pruebas, esa ruta debe entenderse como **flujo auxiliar de testing**. En ese caso, tras enrolar en PostgreSQL, hay que ejecutar una reconstrucción del snapshot local para que el dato aparezca en la DB local usada por la app.

Ejemplo mínimo de bindings relevantes:

```/dev/null/ZkfpLibrary.java#L1-32
public interface ZkfpLibrary extends com.sun.jna.Library {
    int zkfp_db_open(String dbPath);
    void zkfp_db_close();

    int zkfp_db_bulk_begin();
    int zkfp_db_bulk_row_begin();
    int zkfp_db_bulk_row_add_value(String columnName, int valueType, String value, byte[] blobData, long blobSize);
    int zkfp_db_bulk_row_insert(String tableName, com.sun.jna.ptr.LongByReference outId);
    int zkfp_db_bulk_commit();
    int zkfp_db_bulk_rollback();

    int zkfp_gallery_load_from_db(String tableName, String userIdColumn, String templateColumn);

    int zkfp_sync_config_reset();
    int zkfp_sync_set_postgres_url(String postgresUrl);
    int zkfp_sync_set_manual();
    int zkfp_sync_set_daily_time(byte hour, byte minute);
    int zkfp_sync_add_mapping(String postgresQuery, String localTable, String mappingsJson, int strategy, String uniqueKeysCsv);
    int zkfp_sync_apply_config();
    int zkfp_sync_start();
    int zkfp_sync_stop();
    int zkfp_sync_run_now();
    int zkfp_sync_is_running();
    com.sun.jna.Pointer zkfp_sync_get_last_sync_at();

    com.sun.jna.Pointer zkfp_get_last_error();
    void zkfp_free_string(com.sun.jna.Pointer value);
}
```

Uso orientativo para snapshot local offline:

```/dev/null/SyncExample.java#L1-42
ZkfpLibrary lib = com.sun.jna.Native.load("zkfp_capi", ZkfpLibrary.class);
com.sun.jna.ptr.LongByReference outId = new com.sun.jna.ptr.LongByReference();
byte[] tmpl = new byte[] {1, 2, 3, 4, 5};

if (lib.zkfp_db_open("fingerprint.db") == 0) {
    throw new IllegalStateException("DB open failed");
}

lib.zkfp_db_bulk_begin();

lib.zkfp_db_bulk_row_begin();
lib.zkfp_db_bulk_row_add_value("id", 0, "1", null, 0);
lib.zkfp_db_bulk_row_add_value("name", 2, "Alice", null, 0);
lib.zkfp_db_bulk_row_insert("users", outId);

lib.zkfp_db_bulk_row_begin();
lib.zkfp_db_bulk_row_add_value("id", 0, "1", null, 0);
lib.zkfp_db_bulk_row_add_value("user_id", 0, "1", null, 0);
lib.zkfp_db_bulk_row_add_value("finger", 2, "right_index", null, 0);
lib.zkfp_db_bulk_row_add_value("template_data", 3, null, tmpl, tmpl.length);
lib.zkfp_db_bulk_row_add_value("quality", 0, "50", null, 0);
lib.zkfp_db_bulk_row_insert("templates", outId);

lib.zkfp_db_bulk_commit();
lib.zkfp_gallery_load_from_db("templates", "user_id", "template_data");

lib.zkfp_db_close();
```

### Cadena de conexión PostgreSQL

La API espera el formato de conexión aceptado por `tokio-postgres`, por ejemplo:

```/dev/null/postgres.txt#L1-1
host=127.0.0.1 port=5432 user=postgres password=secret dbname=zkfp
```

No hardcodees credenciales en producción.

---

## Umbrella header `zkfp.h`

Si quieres incluir todo el SDK de una vez:

```/home/victormanuelabadpereda/PROYECTOS/fingerprint/crates/zkfp-capi/include/zkfp.h#L1-11
#ifndef ZKFP_H
#define ZKFP_H

#include "zkfp_common.h"
#include "zkfp_usb.h"
#include "zkfp_image.h"
#include "zkfp_template.h"
#include "zkfp_match.h"
#include "zkfp_db.h"
#include "zkfp_sync.h"

#endif
```

---

## Manejo de memoria

### `zkfp_free_string`

Úsalo para liberar cualquier `char*` devuelto por:

- `zkfp_capture_image_base64`
- `zkfp_image_file_to_base64` mediante `out_base64`
- `zkfp_capture_full` mediante `out_base64_png`
- `zkfp_extract_from_image_file` mediante `out_base64_png`
- `zkfp_extract_from_bmp_file` mediante `out_base64_png`
- todas las funciones `zkfp_db_*_json`
- `zkfp_sync_get_last_sync_at`

### `zkfp_free_template`

Úsalo para liberar buffers creados en:

- `zkfp_extract_template`
- `zkfp_capture_full`
- `zkfp_capture_and_extract_template`
- `zkfp_extract_from_image_file`
- `zkfp_extract_from_bmp_file`

---

## Manejo de errores

| Función | Descripción |
|---|---|
| `zkfp_get_last_error()` | Devuelve el último error del hilo actual. |

### Ejemplo

```/dev/null/example.c#L1-8
if (!zkfp_db_open("fingerprint.db")) {
    const char* err = zkfp_get_last_error();
    fprintf(stderr, "DB error: %s\n", err);
}
```

---

## Seguridad de hilos

- el scanner está protegido por `Mutex`
- la DB global está protegida por `Mutex`
- la galería en memoria está protegida por `Mutex`
- el matcher está protegido por `Mutex`
- el extractor NBIS está protegido por `Mutex`
- `zkfp_get_last_error()` usa almacenamiento thread-local

El SDK es seguro para uso concurrente a nivel interno, pero semánticamente sigue trabajando con **estado global compartido**.

---

## Compilación

### Debug

```/dev/null/build.sh#L1-1
cargo build -p zkfp-capi
```

### Release

```/dev/null/build.sh#L1-1
cargo build --release -p zkfp-capi
```

Salida esperada:

- `target/debug/libzkfp_capi.so`
- `target/release/libzkfp_capi.so`

---

## Integración mínima en C

```/dev/null/example.c#L1-34
#include <stdio.h>
#include "zkfp.h"

int main(void) {
    int64_t row_id = 0;
    ZkfpTemplate tmpl = {0};

    if (!zkfp_db_open("fingerprint.db")) {
        fprintf(stderr, "DB error: %s\n", zkfp_get_last_error());
        return 1;
    }

    if (!zkfp_init()) {
        fprintf(stderr, "USB error: %s\n", zkfp_get_last_error());
        return 1;
    }

    if (!zkfp_capture_and_extract_template(&tmpl)) {
        fprintf(stderr, "Capture error: %s\n", zkfp_get_last_error());
        return 1;
    }

    if (!zkfp_db_create_fingerprint_schema("users", "templates", "name", "user_id", "finger", "template_data", "quality")) {
        fprintf(stderr, "Schema error: %s\n", zkfp_get_last_error());
        return 1;
    }

    zkfp_free_template(&tmpl);
    zkfp_close();
    zkfp_db_close();
    return 0;
}
```
