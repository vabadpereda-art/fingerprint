#ifndef ZKFP_DB_H
#define ZKFP_DB_H

#include "zkfp_common.h" // IWYU pragma: export

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

/* data_type values:
 * 0=Integer, 1=Real, 2=Text, 3=Blob, 4=Boolean, 5=Timestamp
 * constraint_flags bitmask:
 * 0x01=PrimaryKey, 0x02=NotNull, 0x04=Unique, 0x08=AutoIncrement, 0x10=ForeignKey
 * value_type values:
 * 0=Integer, 1=Real, 2=Text, 3=Boolean, 4=Null, 5=JSON
 * JSON-returning functions must be freed with zkfp_free_string().
 */

#ifdef __cplusplus
}
#endif

#endif
