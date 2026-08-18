package com.zkfp;

import com.sun.jna.Pointer;
import com.sun.jna.ptr.LongByReference;
import java.nio.file.Files;
import java.nio.file.Path;
import org.json.JSONArray;
import org.json.JSONObject;

public class NativeDbSync {

    public static final int DATA_TYPE_INTEGER = 0;
    public static final int DATA_TYPE_REAL = 1;
    public static final int DATA_TYPE_TEXT = 2;
    public static final int DATA_TYPE_BLOB = 3;
    public static final int DATA_TYPE_BOOLEAN = 4;
    public static final int DATA_TYPE_TIMESTAMP = 5;

    public static final int CONSTRAINT_PRIMARY_KEY = 0x01;
    public static final int CONSTRAINT_NOT_NULL = 0x02;
    public static final int CONSTRAINT_UNIQUE = 0x04;
    public static final int CONSTRAINT_AUTO_INCREMENT = 0x08;
    public static final int CONSTRAINT_FOREIGN_KEY = 0x10;

    public static final int SYNC_REPLACE = 0;
    public static final int SYNC_APPEND = 1;
    public static final int SYNC_UPSERT = 2;

    private final ZkfpLibrary lib;

    public NativeDbSync(ZkfpLibrary lib) {
        this.lib = lib;
    }

    public void openLocalDb(String dbPath) {
        ensure(lib.zkfp_db_open(dbPath), "Failed to open local DB");
    }

    public void closeLocalDb() {
        lib.zkfp_db_close();
    }

    public void ensureAppSchema() {
        String tablesJson = listTablesJson();
        JSONArray tables = new JSONArray(
                tablesJson == null || tablesJson.isBlank() ? "[]" : tablesJson);

        boolean hasUsers = false;
        boolean hasTemplates = false;
        for (int i = 0; i < tables.length(); i++) {
            String table = tables.getString(i);
            if ("users".equals(table)) {
                hasUsers = true;
            } else if ("templates".equals(table)) {
                hasTemplates = true;
            }
        }

        if (hasUsers && hasTemplates) {
            return;
        }

        ensure(
                lib.zkfp_db_create_fingerprint_schema(
                        "users",
                        "templates",
                        "name",
                        "user_id",
                        "finger",
                        "template_data",
                        "quality"),
                "Failed to create fingerprint schema");
    }

    public long insertUserJson(String json) {
        LongByReference outId = new LongByReference();
        ensure(
                lib.zkfp_db_insert_json("users", json, outId),
                "Failed to insert user JSON");
        return outId.getValue();
    }

    public long insertTemplateJson(String json) {
        LongByReference outId = new LongByReference();
        ensure(
                lib.zkfp_db_insert_json("templates", json, outId),
                "Failed to insert template JSON");
        return outId.getValue();
    }

    public void rebuildLocalSnapshot(
            String dbPath,
            java.util.List<FingerprintRepository.UserRecord> users,
            java.util.List<FingerprintRepository.TemplateRecord> templates) {
        closeLocalDb();
        try {
            Files.deleteIfExists(Path.of(dbPath));
        } catch (Exception e) {
            throw new IllegalStateException(
                    "Failed to delete local DB file: " + e.getMessage(),
                    e);
        }

        openLocalDb(dbPath);
        ensureAppSchema();

        ensure(
                lib.zkfp_db_bulk_begin(),
                "Failed to begin bulk insert transaction");
        try {
            for (FingerprintRepository.UserRecord user : users) {
                bulkRowBegin();
                bulkRowAddInteger("id", user.id);
                bulkRowAddText("name", user.name);
                bulkRowInsert("users");
            }

            for (FingerprintRepository.TemplateRecord template : templates) {
                bulkRowBegin();
                bulkRowAddInteger("id", template.id);
                bulkRowAddInteger("user_id", template.userId);
                bulkRowAddText(
                        "finger",
                        template.fingerPosition != null
                                ? template.fingerPosition
                                : "unknown");
                bulkRowAddBlob("template_data", template.templateData);
                bulkRowAddInteger("quality", 0);
                bulkRowInsert("templates");
            }

            ensure(
                    lib.zkfp_db_bulk_commit(),
                    "Failed to commit bulk insert transaction");
        } catch (RuntimeException e) {
            lib.zkfp_db_bulk_rollback();
            throw e;
        }
    }

    public long createUser(String name) {
        JSONObject json = new JSONObject();
        json.put("name", name);
        return insertUserJson(json.toString());
    }

    public long createTemplate(
            long userId,
            String finger,
            byte[] templateData,
            int quality) {
        JSONObject json = new JSONObject();
        json.put("user_id", userId);
        json.put("finger", finger);
        json.put("template_data", toJsonByteArray(templateData));
        json.put("quality", quality);
        return insertTemplateJson(json.toString());
    }

    public String getUserNameById(long userId) {
        String json = queryEqJson(
                "users",
                "id",
                DATA_TYPE_INTEGER,
                Long.toString(userId));
        JSONArray rows = new JSONArray(
                json == null || json.isBlank() ? "[]" : json);
        if (rows.isEmpty()) {
            return null;
        }
        return rows.getJSONObject(0).optString("name", null);
    }

    public JSONArray listTemplates() {
        String json = listRowsJson("templates");
        return new JSONArray(json == null || json.isBlank() ? "[]" : json);
    }

    public JSONArray listUsers() {
        String json = listRowsJson("users");
        return new JSONArray(json == null || json.isBlank() ? "[]" : json);
    }

    public void clearUsersAndTemplates() {
        JSONArray templates = listTemplates();
        for (int i = 0; i < templates.length(); i++) {
            JSONObject row = templates.getJSONObject(i);
            ensure(
                    lib.zkfp_db_delete_row("templates", row.getLong("id")),
                    "Failed to delete local template");
        }

        JSONArray users = listUsers();
        for (int i = 0; i < users.length(); i++) {
            JSONObject row = users.getJSONObject(i);
            ensure(
                    lib.zkfp_db_delete_row("users", row.getLong("id")),
                    "Failed to delete local user");
        }
    }

    public String listRowsJson(String tableName) {
        Pointer ptr = lib.zkfp_db_list_rows_json(tableName);
        return readAndFreeString(ptr);
    }

    public String getRowJson(String tableName, long rowId) {
        Pointer ptr = lib.zkfp_db_get_row_json(tableName, rowId);
        return readAndFreeString(ptr);
    }

    public String listTablesJson() {
        Pointer ptr = lib.zkfp_db_list_tables_json();
        return readAndFreeString(ptr);
    }

    public String getSchemaJson(String tableName) {
        Pointer ptr = lib.zkfp_db_get_schema_json(tableName);
        return readAndFreeString(ptr);
    }

    public String queryEqJson(
            String tableName,
            String columnName,
            int valueType,
            String value) {
        Pointer ptr = lib.zkfp_db_query_eq_json(
                tableName,
                columnName,
                valueType,
                value);
        return readAndFreeString(ptr);
    }

    public String queryLikeJson(
            String tableName,
            String columnName,
            String pattern) {
        Pointer ptr = lib.zkfp_db_query_like_json(
                tableName,
                columnName,
                pattern);
        return readAndFreeString(ptr);
    }

    public long count(String tableName) {
        return lib.zkfp_db_count(tableName);
    }

    public void configureManualSync(String postgresUrl) {
        ensure(lib.zkfp_sync_config_reset(), "Failed to reset sync config");
        ensure(
                lib.zkfp_sync_set_postgres_url(postgresUrl),
                "Failed to set PostgreSQL URL");
        ensure(lib.zkfp_sync_set_manual(), "Failed to set manual sync mode");
    }

    public void configureDailySync(String postgresUrl, byte hour, byte minute) {
        ensure(lib.zkfp_sync_config_reset(), "Failed to reset sync config");
        ensure(
                lib.zkfp_sync_set_postgres_url(postgresUrl),
                "Failed to set PostgreSQL URL");
        ensure(
                lib.zkfp_sync_set_daily_time(hour, minute),
                "Failed to set daily sync time");
    }

    public void configureIntervalSync(String postgresUrl, int seconds) {
        ensure(lib.zkfp_sync_config_reset(), "Failed to reset sync config");
        ensure(
                lib.zkfp_sync_set_postgres_url(postgresUrl),
                "Failed to set PostgreSQL URL");
        ensure(
                lib.zkfp_sync_set_interval_seconds(seconds),
                "Failed to set sync interval");
    }

    public void addUsersSyncMapping() {
        ensure(
                lib.zkfp_sync_add_mapping(
                        "SELECT id, name FROM users",
                        "users",
                        "{\"id\":\"id\",\"name\":\"name\"}",
                        SYNC_UPSERT,
                        "id"),
                "Failed to add users sync mapping");
    }

    public void addTemplatesSyncMapping() {
        ensure(
                lib.zkfp_sync_add_mapping(
                        "SELECT user_id, finger_position, template_data FROM templates",
                        "templates",
                        "{\"user_id\":\"user_id\",\"finger_position\":\"finger\",\"template_data\":\"template_data\"}",
                        SYNC_APPEND,
                        "user_id,finger"),
                "Failed to add templates sync mapping");
    }

    public void applySyncConfig() {
        ensure(lib.zkfp_sync_apply_config(), "Failed to apply sync config");
    }

    public void runSyncNow() {
        ensure(lib.zkfp_sync_run_now(), "Failed to run sync now");
    }

    public void startScheduler() {
        ensure(lib.zkfp_sync_start(), "Failed to start sync scheduler");
    }

    public void stopScheduler() {
        ensure(lib.zkfp_sync_stop(), "Failed to stop sync scheduler");
    }

    public boolean isSyncConfigured() {
        return lib.zkfp_sync_is_running() == 1;
    }

    public String getLastSyncAt() {
        Pointer ptr = lib.zkfp_sync_get_last_sync_at();
        return readAndFreeString(ptr);
    }

    public String getLastError() {
        return lib.zkfp_get_last_error();
    }

    private String readAndFreeString(Pointer ptr) {
        if (ptr == null) {
            return null;
        }
        String value = ptr.getString(0);
        lib.zkfp_free_string(ptr);
        return value;
    }

    private void bulkRowBegin() {
        ensure(lib.zkfp_db_bulk_row_begin(), "Failed to begin bulk row");
    }

    private void bulkRowAddInteger(String columnName, long value) {
        ensure(
                lib.zkfp_db_bulk_row_add_value(
                        columnName,
                        DATA_TYPE_INTEGER,
                        Long.toString(value),
                        null,
                        0),
                "Failed to add integer column '" + columnName + "'");
    }

    private void bulkRowAddText(String columnName, String value) {
        ensure(
                lib.zkfp_db_bulk_row_add_value(
                        columnName,
                        DATA_TYPE_TEXT,
                        value,
                        null,
                        0),
                "Failed to add text column '" + columnName + "'");
    }

    private void bulkRowAddBlob(String columnName, byte[] value) {
        ensure(
                lib.zkfp_db_bulk_row_add_value(
                        columnName,
                        DATA_TYPE_BLOB,
                        null,
                        value,
                        value.length),
                "Failed to add blob column '" + columnName + "'");
    }

    private void bulkRowInsert(String tableName) {
        LongByReference outId = new LongByReference();
        ensure(
                lib.zkfp_db_bulk_row_insert(tableName, outId),
                "Failed typed bulk insert into table '" + tableName + "'");
    }

    private JSONArray toJsonByteArray(byte[] data) {
        JSONArray array = new JSONArray();
        for (byte b : data) {
            array.put(Byte.toUnsignedInt(b));
        }
        return array;
    }

    private void ensure(int result, String message) {
        if (result != 1) {
            String nativeError = lib.zkfp_get_last_error();
            if (nativeError == null || nativeError.isBlank()) {
                throw new IllegalStateException(message);
            }
            throw new IllegalStateException(message + ": " + nativeError);
        }
    }
}
