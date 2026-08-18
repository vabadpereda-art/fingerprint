package com.zkfp;

import com.zaxxer.hikari.HikariConfig;
import com.zaxxer.hikari.HikariDataSource;
import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.Statement;
import java.util.ArrayList;
import java.util.List;

public class FingerprintRepository {

    private final HikariDataSource dataSource;

    public FingerprintRepository(String jdbcUrl, String user, String password) {
        HikariConfig config = new HikariConfig();
        config.setJdbcUrl(jdbcUrl);
        config.setUsername(user);
        config.setPassword(password);
        this.dataSource = new HikariDataSource(config);
        initDatabase();
    }

    private void initDatabase() {
        String createUsersTable = "CREATE TABLE IF NOT EXISTS users (" +
                "id SERIAL PRIMARY KEY, " +
                "name VARCHAR(255) NOT NULL, " +
                "created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)";

        String createTemplatesTable = "CREATE TABLE IF NOT EXISTS templates (" +
                "id SERIAL PRIMARY KEY, " +
                "user_id INTEGER REFERENCES users(id) ON DELETE CASCADE, " +
                "finger_position VARCHAR(50), " +
                "template_data BYTEA NOT NULL, " +
                "created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)";

        try (
                Connection conn = dataSource.getConnection();
                Statement stmt = conn.createStatement()) {
            stmt.execute(createUsersTable);
            stmt.execute(createTemplatesTable);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public int addUser(String name) {
        String sql = "INSERT INTO users (name) VALUES (?) RETURNING id";
        try (
                Connection conn = dataSource.getConnection();
                PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setString(1, name);
            ResultSet rs = pstmt.executeQuery();
            if (rs.next()) {
                return rs.getInt("id");
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return -1;
    }

    public void saveTemplate(
            int userId,
            String fingerPosition,
            byte[] templateData) {
        String sql = "INSERT INTO templates (user_id, finger_position, template_data) VALUES (?, ?, ?)";
        try (
                Connection conn = dataSource.getConnection();
                PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setInt(1, userId);
            pstmt.setString(2, fingerPosition);
            pstmt.setBytes(3, templateData);
            pstmt.executeUpdate();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void deleteUser(int userId) {
        String sql = "DELETE FROM users WHERE id = ?";
        try (
                Connection conn = dataSource.getConnection();
                PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setInt(1, userId);
            pstmt.executeUpdate();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public String getUserNameById(int userId) {
        String sql = "SELECT name FROM users WHERE id = ?";
        try (
                Connection conn = dataSource.getConnection();
                PreparedStatement pstmt = conn.prepareStatement(sql)) {
            pstmt.setInt(1, userId);
            try (ResultSet rs = pstmt.executeQuery()) {
                if (rs.next()) {
                    return rs.getString("name");
                }
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return null;
    }

    public void clearAll() {
        try (
                Connection conn = dataSource.getConnection();
                Statement stmt = conn.createStatement()) {
            stmt.execute("TRUNCATE TABLE users CASCADE");
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public static class UserRecord {

        public int id;
        public String name;
    }

    public static class TemplateRecord {

        public int id;
        public int userId;
        public String fingerPosition;
        public byte[] templateData;
    }

    public List<UserRecord> loadAllUsers() {
        List<UserRecord> records = new ArrayList<>();
        String sql = "SELECT id, name FROM users ORDER BY id";
        try (
                Connection conn = dataSource.getConnection();
                PreparedStatement pstmt = conn.prepareStatement(sql);
                ResultSet rs = pstmt.executeQuery()) {
            while (rs.next()) {
                UserRecord ur = new UserRecord();
                ur.id = rs.getInt("id");
                ur.name = rs.getString("name");
                records.add(ur);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return records;
    }

    public List<TemplateRecord> loadAllTemplates() {
        List<TemplateRecord> records = new ArrayList<>();
        String sql = "SELECT id, user_id, finger_position, template_data FROM templates ORDER BY id";
        try (
                Connection conn = dataSource.getConnection();
                PreparedStatement pstmt = conn.prepareStatement(sql);
                ResultSet rs = pstmt.executeQuery()) {
            while (rs.next()) {
                TemplateRecord tr = new TemplateRecord();
                tr.id = rs.getInt("id");
                tr.userId = rs.getInt("user_id");
                tr.fingerPosition = rs.getString("finger_position");
                tr.templateData = rs.getBytes("template_data");
                records.add(tr);
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
        return records;
    }
}
