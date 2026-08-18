package com.zkfp;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import com.sun.jna.ptr.IntByReference;
import com.sun.jna.ptr.LongByReference;
import com.sun.jna.ptr.PointerByReference;

public interface ZkfpLibrary extends Library {
    ZkfpLibrary INSTANCE = Native.load("zkfp_capi", ZkfpLibrary.class);

    @Structure.FieldOrder(
        {
            "apply_enhancement",
            "method",
            "bg_intensity",
            "invert",
            "flip_vertical",
            "padding",
        }
    )
    class ZkfpEnhanceConfig extends Structure {

        public int apply_enhancement;
        public int method;
        public byte bg_intensity;
        public int invert;
        public int flip_vertical;
        public int padding;

        public ZkfpEnhanceConfig() {}
    }

    @Structure.FieldOrder({ "data", "size", "quality" })
    class ZkfpTemplate extends Structure {

        public Pointer data;
        public long size;
        public int quality;

        public ZkfpTemplate() {}

        public byte[] getDataBytes() {
            if (data != null && size > 0) {
                return data.getByteArray(0, (int) size);
            }
            return new byte[0];
        }
    }

    @Structure.FieldOrder(
        {
            "user_id",
            "identify_score",
            "verify_score",
            "identify_match",
            "verify_match",
        }
    )
    class ZkfpIdentifyVerifyResult extends Structure {

        public int user_id;
        public int identify_score;
        public int verify_score;
        public int identify_match;
        public int verify_match;

        public ZkfpIdentifyVerifyResult() {}
    }

    // Hardware Init & Config
    int zkfp_init();

    void zkfp_close();

    String zkfp_get_last_error();

    // Image Capture & Base64
    Pointer zkfp_capture_image_base64(String format);

    void zkfp_free_string(Pointer s);

    // Image Enhancement Configuration
    int zkfp_set_enhance_config(ZkfpEnhanceConfig config);

    int zkfp_get_enhance_config(ZkfpEnhanceConfig config);

    int zkfp_set_contrast_method(int method);

    int zkfp_set_invert(int invert);

    int zkfp_set_flip_vertical(int flip);

    int zkfp_set_bg_intensity(byte intensity);

    int zkfp_set_padding(int padding);

    int zkfp_set_enhancement_enabled(int enabled);

    // Template Operations — single capture, returns template + optional base64 PNG
    int zkfp_capture_full(
        ZkfpTemplate out_template,
        PointerByReference out_base64_png
    );

    int zkfp_capture_and_extract_template(ZkfpTemplate out_template); // legacy (no image)

    int zkfp_extract_template(
        byte[] bmp_data,
        long bmp_size,
        ZkfpTemplate out_template
    );

    int zkfp_extract_from_bmp_file(
        String path,
        ZkfpTemplate out_template,
        PointerByReference out_base64_png
    );

    int zkfp_extract_from_image_file(
        String path,
        ZkfpTemplate out_template,
        PointerByReference out_base64_png
    );

    int zkfp_image_file_to_base64(
        String path,
        String format,
        PointerByReference out_base64
    );

    void zkfp_free_template(ZkfpTemplate template);

    // 1:1 Matching
    int zkfp_verify_templates(
        byte[] tmpl1_data,
        long tmpl1_size,
        byte[] tmpl2_data,
        long tmpl2_size
    );

    // Memory Gallery Operations
    void zkfp_gallery_clear();

    int zkfp_gallery_add(int user_id, byte[] tmpl_data, long tmpl_size);

    int zkfp_gallery_load_from_db(
        String tableName,
        String userIdColumn,
        String templateColumn
    );

    void zkfp_gallery_remove(int user_id);

    int zkfp_gallery_identify(
        byte[] probe_data,
        long probe_size,
        IntByReference out_id,
        IntByReference out_score
    );

    int zkfp_gallery_identify_with_verification(
        byte[] probe_data,
        long probe_size,
        ZkfpIdentifyVerifyResult out_result
    );

    // Native DB API
    int zkfp_db_open(String dbPath);

    void zkfp_db_close();

    int zkfp_db_register_table(String tableName);

    int zkfp_db_create_fingerprint_schema(
        String usersTable,
        String templatesTable,
        String userNameColumn,
        String templateUserIdColumn,
        String templateFingerColumn,
        String templateDataColumn,
        String templateQualityColumn
    );

    int zkfp_db_add_column_and_create(
        String tableName,
        String columnName,
        int dataType,
        int constraintFlags,
        String foreignTable,
        String foreignColumn
    );

    int zkfp_db_insert_kv(
        String tableName,
        String columnName,
        int valueType,
        String value,
        LongByReference outId
    );

    int zkfp_db_insert_json(
        String tableName,
        String jsonObject,
        LongByReference outId
    );

    int zkfp_db_update_kv(
        String tableName,
        long rowId,
        String columnName,
        int valueType,
        String value
    );

    int zkfp_db_update_json(String tableName, long rowId, String jsonObject);

    int zkfp_db_delete_row(String tableName, long rowId);

    Pointer zkfp_db_get_row_json(String tableName, long rowId);

    Pointer zkfp_db_list_rows_json(String tableName);

    Pointer zkfp_db_query_eq_json(
        String tableName,
        String columnName,
        int valueType,
        String value
    );

    Pointer zkfp_db_query_like_json(
        String tableName,
        String columnName,
        String pattern
    );

    Pointer zkfp_db_list_tables_json();

    Pointer zkfp_db_get_schema_json(String tableName);

    long zkfp_db_count(String tableName);

    int zkfp_db_delete_all_rows(String tableName);

    int zkfp_db_bulk_begin();

    int zkfp_db_bulk_insert_json(
        String tableName,
        String jsonObject,
        LongByReference outId
    );

    int zkfp_db_bulk_row_begin();

    int zkfp_db_bulk_row_add_value(
        String columnName,
        int valueType,
        String value,
        byte[] blobData,
        long blobSize
    );

    int zkfp_db_bulk_row_insert(String tableName, LongByReference outId);

    int zkfp_db_bulk_commit();

    int zkfp_db_bulk_rollback();

    // Sync API
    int zkfp_sync_config_reset();
    int zkfp_sync_set_postgres_url(String postgresUrl);
    int zkfp_sync_set_interval_seconds(int seconds);
    int zkfp_sync_set_daily_time(byte hour, byte minute);
    int zkfp_sync_set_weekly_time(String weekdaysCsv, byte hour, byte minute);
    int zkfp_sync_set_cron(String cronExpression);
    int zkfp_sync_set_manual();
    int zkfp_sync_add_mapping(
        String postgresQuery,
        String localTable,
        String mappingsJson,
        int strategy,
        String uniqueKeysCsv
    );
    int zkfp_sync_apply_config();
    int zkfp_sync_start();
    int zkfp_sync_stop();
    int zkfp_sync_run_now();
    int zkfp_sync_is_running();
    Pointer zkfp_sync_get_last_sync_at();
}
