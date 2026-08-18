package com.zkfp;

import com.formdev.flatlaf.FlatLightLaf;
import com.sun.jna.Pointer;
import com.sun.jna.ptr.PointerByReference;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Base64;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Locale;
import java.util.Set;
import javax.imageio.ImageIO;
import javax.swing.*;
import javax.swing.filechooser.FileNameExtensionFilter;

public class App extends JFrame {

    private static final ZkfpLibrary lib = ZkfpLibrary.INSTANCE;
    private static final Set<String> IMPORT_EXTENSIONS = Set.of(
            "bmp",
            "png",
            "jpg",
            "jpeg",
            "tiff",
            "tif",
            "webp",
            "wsq");

    private static final String LOCAL_DB_PATH = "fingerprint-local.db";
    private static FingerprintRepository db;
    private static NativeDbSync nativeDb;

    // Controls
    private JTextField nameField;
    private JButton enrollScannerBtn;
    private JButton enrollImageBtn;
    private JButton identifyBtn;
    private JButton syncBtn;
    private JButton clearBtn;
    private JButton exportImageBtn;
    private JLabel fingerprintLabel;
    private JTextArea logArea;
    private JLabel statusLabel;
    private BufferedImage currentFingerprintImage;
    private String currentFingerprintBase64;

    public App() {
        setTitle("ZKFP Biometric System");
        setSize(860, 540);
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setLocationRelativeTo(null);
        setLayout(new BorderLayout(8, 8));

        buildTopPanel();
        buildCenterPanel();
        buildStatusBar();

        setAllButtons(false);
        status("Initializing…");
        startInitializationAsync();
    }

    private void buildTopPanel() {
        JPanel nameRow = new JPanel(new FlowLayout(FlowLayout.LEFT, 8, 6));
        nameRow.add(new JLabel("User Name:"));
        nameField = new JTextField(22);
        nameRow.add(nameField);

        enrollScannerBtn = new JButton("Enroll from Scanner");
        enrollImageBtn = new JButton("Enroll from Image…");
        identifyBtn = new JButton("Identify Fingerprint");
        syncBtn = new JButton("Sync from PostgreSQL");
        clearBtn = new JButton("Clear Database");
        exportImageBtn = new JButton("Export Preview…");
        exportImageBtn.setEnabled(false);

        JPanel btnRow = new JPanel(new GridLayout(1, 6, 6, 0));
        btnRow.setBorder(BorderFactory.createEmptyBorder(0, 8, 6, 8));
        btnRow.add(enrollScannerBtn);
        btnRow.add(enrollImageBtn);
        btnRow.add(identifyBtn);
        btnRow.add(syncBtn);
        btnRow.add(clearBtn);
        btnRow.add(exportImageBtn);

        JPanel top = new JPanel(new BorderLayout());
        top.add(nameRow, BorderLayout.NORTH);
        top.add(btnRow, BorderLayout.CENTER);
        add(top, BorderLayout.NORTH);

        enrollScannerBtn.addActionListener(e -> enrollFromScanner());
        enrollImageBtn.addActionListener(e -> enrollFromImage());
        identifyBtn.addActionListener(e -> identify());
        syncBtn.addActionListener(e -> syncFromPostgres());
        clearBtn.addActionListener(e -> clearDatabase());
        exportImageBtn.addActionListener(e -> exportPreview());
    }

    private void buildCenterPanel() {
        JPanel center = new JPanel(new BorderLayout(8, 0));
        center.setBorder(BorderFactory.createEmptyBorder(0, 8, 8, 8));

        fingerprintLabel = new JLabel("No Image", SwingConstants.CENTER);
        fingerprintLabel.setPreferredSize(new Dimension(210, 260));
        fingerprintLabel.setBorder(
                BorderFactory.createTitledBorder("Fingerprint Preview"));
        fingerprintLabel.setBackground(new Color(40, 40, 40));
        fingerprintLabel.setForeground(Color.LIGHT_GRAY);
        fingerprintLabel.setOpaque(true);
        center.add(fingerprintLabel, BorderLayout.WEST);

        logArea = new JTextArea();
        logArea.setEditable(false);
        logArea.setFont(new Font("Monospaced", Font.PLAIN, 12));
        JScrollPane scroll = new JScrollPane(logArea);
        scroll.setBorder(BorderFactory.createTitledBorder("System Log"));
        center.add(scroll, BorderLayout.CENTER);

        add(center, BorderLayout.CENTER);
    }

    private void buildStatusBar() {
        statusLabel = new JLabel("  Ready");
        statusLabel.setBorder(BorderFactory.createEtchedBorder());
        statusLabel.setFont(new Font("SansSerif", Font.PLAIN, 12));
        add(statusLabel, BorderLayout.SOUTH);
    }

    private void log(String msg) {
        SwingUtilities.invokeLater(() -> {
            logArea.append(msg + "\n");
            logArea.setCaretPosition(logArea.getDocument().getLength());
        });
    }

    private void status(String msg) {
        SwingUtilities.invokeLater(() -> statusLabel.setText("  " + msg));
    }

    private void setAllButtons(boolean enabled) {
        SwingUtilities.invokeLater(() -> {
            enrollScannerBtn.setEnabled(enabled);
            enrollImageBtn.setEnabled(enabled);
            identifyBtn.setEnabled(enabled);
            syncBtn.setEnabled(enabled);
            clearBtn.setEnabled(enabled);
            exportImageBtn.setEnabled(enabled && hasCurrentImageData());
        });
    }

    private boolean hasCurrentImageData() {
        return (currentFingerprintImage != null &&
                currentFingerprintBase64 != null &&
                !currentFingerprintBase64.isBlank());
    }

    private void showFingerprintB64(String base64Image) {
        SwingUtilities.invokeLater(() -> {
            try {
                byte[] bytes = Base64.getDecoder().decode(base64Image);
                BufferedImage bi = ImageIO.read(
                        new ByteArrayInputStream(bytes));
                if (bi != null) {
                    currentFingerprintImage = bi;
                    currentFingerprintBase64 = base64Image;
                    Image scaled = bi.getScaledInstance(
                            200,
                            250,
                            Image.SCALE_SMOOTH);
                    fingerprintLabel.setIcon(new ImageIcon(scaled));
                    fingerprintLabel.setText("");
                    exportImageBtn.setEnabled(true);
                } else {
                    log(
                            "Image display error: decoded data is not a supported raster image.");
                }
            } catch (Exception ex) {
                log("Image display error: " + ex.getMessage());
            }
        });
    }

    private void clearPreview() {
        SwingUtilities.invokeLater(() -> {
            currentFingerprintImage = null;
            currentFingerprintBase64 = null;
            fingerprintLabel.setIcon(null);
            fingerprintLabel.setText("No Image");
            exportImageBtn.setEnabled(false);
        });
    }

    private void exportPreview() {
        if (!hasCurrentImageData()) {
            JOptionPane.showMessageDialog(
                    this,
                    "There is no preview image to export.",
                    "No Image",
                    JOptionPane.WARNING_MESSAGE);
            return;
        }

        String[] exportFormats = {
                "png",
                "bmp",
                "jpg",
                "tiff",
                "webp",
                "wsq",
                "txt",
        };
        JComboBox<String> formatCombo = new JComboBox<>(exportFormats);
        formatCombo.setSelectedItem("png");

        JFileChooser chooser = new JFileChooser();
        chooser.setDialogTitle("Export Fingerprint Preview");
        chooser.setAcceptAllFileFilterUsed(false);
        chooser.setFileFilter(
                new FileNameExtensionFilter(
                        "Supported export formats",
                        exportFormats));
        chooser.setSelectedFile(new File("fingerprint_preview.png"));

        JPanel accessory = new JPanel(new BorderLayout(6, 6));
        accessory.setBorder(BorderFactory.createEmptyBorder(8, 8, 8, 8));
        accessory.add(new JLabel("Export format:"), BorderLayout.NORTH);
        accessory.add(formatCombo, BorderLayout.CENTER);
        chooser.setAccessory(accessory);

        formatCombo.addActionListener(e -> {
            String selectedFormat = (String) formatCombo.getSelectedItem();
            if (selectedFormat == null || selectedFormat.isBlank()) {
                return;
            }
            File current = chooser.getSelectedFile();
            if (current == null) {
                chooser.setSelectedFile(
                        new File("fingerprint_preview." + selectedFormat));
            } else {
                chooser.setSelectedFile(
                        ensureExtension(
                                removeKnownExtension(current),
                                selectedFormat));
            }
        });

        if (chooser.showSaveDialog(this) != JFileChooser.APPROVE_OPTION) {
            return;
        }

        String extension = ((String) formatCombo.getSelectedItem());
        if (extension == null || extension.isBlank()) {
            extension = getExtension(chooser.getSelectedFile().getName());
        }
        if (extension == null || extension.isBlank()) {
            extension = "png";
        }

        File file = ensureExtension(
                removeKnownExtension(chooser.getSelectedFile()),
                extension);

        try {
            exportCurrentImage(file, extension);
            String msg = "Preview exported as " +
                    extension.toUpperCase(Locale.ROOT) +
                    ": " +
                    file.getAbsolutePath();
            log(msg);
            status(msg);
        } catch (Exception ex) {
            String msg = "Failed to export preview: " + ex.getMessage();
            log(msg);
            status("Failed to export preview.");
            JOptionPane.showMessageDialog(
                    this,
                    msg,
                    "Export Error",
                    JOptionPane.ERROR_MESSAGE);
        }
    }

    private void exportCurrentImage(File file, String extension)
            throws IOException {
        String normalized = extension.toLowerCase(Locale.ROOT);
        String exportedBase64;

        switch (normalized) {
            case "txt" -> {
                exportedBase64 = currentFingerprintBase64;
                Files.writeString(
                        file.toPath(),
                        exportedBase64,
                        StandardCharsets.UTF_8);
            }
            case "jpg", "jpeg" -> {
                exportedBase64 = writeRasterImageAndReturnBase64(file, "jpeg");
            }
            case "tif", "tiff" -> {
                exportedBase64 = writeRasterImageAndReturnBase64(file, "tiff");
            }
            case "png", "bmp", "webp" -> {
                exportedBase64 = writeRasterImageAndReturnBase64(
                        file,
                        normalized);
            }
            case "wsq" -> {
                exportedBase64 = exportWsqAndReturnBase64(file);
            }
            default -> throw new IOException(
                    "Unsupported export format: " + extension);
        }

        log(
                "Exported Base64 (" +
                        normalized.toUpperCase(Locale.ROOT) +
                        "): " +
                        exportedBase64);
    }

    private String writeRasterImageAndReturnBase64(File file, String formatName)
            throws IOException {
        byte[] bytes = encodeCurrentImage(formatName);
        Files.write(file.toPath(), bytes);
        return Base64.getEncoder().encodeToString(bytes);
    }

    private String exportWsqAndReturnBase64(File file) throws IOException {
        PointerByReference b64Ref = new PointerByReference();

        File tempImage = File.createTempFile("zkfp_export_", ".png");
        try {
            byte[] bytes = Base64.getDecoder().decode(currentFingerprintBase64);
            Files.write(tempImage.toPath(), bytes);

            lib.zkfp_set_enhancement_enabled(0);
            int res = lib.zkfp_image_file_to_base64(
                    tempImage.getAbsolutePath(),
                    "wsq",
                    b64Ref);
            lib.zkfp_set_enhancement_enabled(1);

            if (res != 1) {
                throw new IOException(
                        "Native WSQ export failed: " + lib.zkfp_get_last_error());
            }

            return writeNativeBase64ToFile(file, b64Ref);
        } finally {
            tempImage.delete();
        }
    }

    private String writeNativeBase64ToFile(File file, PointerByReference b64Ref)
            throws IOException {
        Pointer p = b64Ref.getValue();
        if (p == null) {
            throw new IOException("Native export returned null Base64.");
        }

        String exportedBase64 = p.getString(0);
        lib.zkfp_free_string(p);
        byte[] bytes = Base64.getDecoder().decode(exportedBase64);
        Files.write(file.toPath(), bytes);
        return exportedBase64;
    }

    private byte[] encodeCurrentImage(String formatName) throws IOException {
        java.io.ByteArrayOutputStream buffer = new java.io.ByteArrayOutputStream();
        boolean ok = ImageIO.write(currentFingerprintImage, formatName, buffer);
        if (!ok) {
            throw new IOException(
                    "No ImageIO writer available for format '" + formatName + "'.");
        }
        return buffer.toByteArray();
    }

    private File removeKnownExtension(File file) {
        String name = file.getName();
        String extension = getExtension(name);
        if (extension == null) {
            return file;
        }

        Set<String> knownExtensions = Set.of(
                "png",
                "bmp",
                "jpg",
                "jpeg",
                "tif",
                "tiff",
                "webp",
                "wsq",
                "txt");
        if (!knownExtensions.contains(extension)) {
            return file;
        }

        String baseName = name.substring(
                0,
                name.length() - extension.length() - 1);
        File parent = file.getParentFile();
        if (parent == null) {
            return new File(baseName);
        }
        return new File(parent, baseName);
    }

    private File ensureExtension(File file, String extension) {
        String expected = "." + extension.toLowerCase(Locale.ROOT);
        String name = file.getName().toLowerCase(Locale.ROOT);
        if (name.endsWith(expected)) {
            return file;
        }
        File parent = file.getParentFile();
        if (parent == null) {
            return new File(file.getPath() + expected);
        }
        return new File(parent, file.getName() + expected);
    }

    private String getExtension(String fileName) {
        int idx = fileName.lastIndexOf('.');
        if (idx < 0 || idx == fileName.length() - 1) {
            return null;
        }
        return fileName.substring(idx + 1).toLowerCase(Locale.ROOT);
    }

    private void handleB64(PointerByReference b64Ref) {
        if (b64Ref == null) {
            return;
        }
        Pointer p = b64Ref.getValue();
        if (p != null) {
            showFingerprintB64(p.getString(0));
            lib.zkfp_free_string(p);
        }
    }

    private void startInitializationAsync() {
        new Thread(
                () -> {
                    initSystem();
                    setAllButtons(true);
                },
                "zkfp-init")
                .start();
    }

    private void initSystem() {
        log("Connecting to Postgres…");
        try {
            db = new FingerprintRepository(
                    "jdbc:postgresql://localhost:5434/fingerprints",
                    "postgres",
                    "postgres");
            log("DB connected.");
        } catch (Exception e) {
            log("DB error: " + e.getMessage());
            return;
        }

        log("Opening local SDK database…");
        try {
            log("Creating NativeDbSync wrapper…");
            nativeDb = new NativeDbSync(lib);
            rebuildLocalSnapshotWithMetrics("initialization");
        } catch (Exception e) {
            log("Local SDK DB/sync error: " + e.getMessage());
            status("Local SDK DB init failed.");
            return;
        }

        log("Initializing ZK9500…");
        if (lib.zkfp_init() != 1) {
            log("Scanner init failed: " + lib.zkfp_get_last_error());
            status("Scanner init failed.");
            return;
        }
        log("Scanner initialized.");
        loadDatabaseToRAM();
        status("Ready");
    }

    private void loadDatabaseToRAM() {
        log("Loading templates from local SDK DB → RAM…");
        int loaded = lib.zkfp_gallery_load_from_db(
                "templates",
                "user_id",
                "template_data");
        if (loaded < 0) {
            throw new IllegalStateException(lib.zkfp_get_last_error());
        }
        log("Loaded " + loaded + " template(s) into RAM cache.");
    }

    private void enrollFromScanner() {
        String name = nameField.getText().trim();
        if (name.isEmpty()) {
            JOptionPane.showMessageDialog(
                    this,
                    "Enter a user name first.",
                    "Error",
                    JOptionPane.ERROR_MESSAGE);
            return;
        }
        setAllButtons(false);
        clearPreview();
        log("Place your finger on the scanner…");
        status("Waiting for finger…");

        new Thread(() -> {
            ZkfpLibrary.ZkfpTemplate tmpl = new ZkfpLibrary.ZkfpTemplate();
            PointerByReference b64Ref = new PointerByReference();

            int res = lib.zkfp_capture_full(tmpl, b64Ref);
            if (res == 1) {
                handleB64(b64Ref);
                enrollTemplate(name, "scanner", tmpl);
            } else {
                log("Capture failed: " + lib.zkfp_get_last_error());
                status("Enrollment failed.");
            }
            lib.zkfp_free_template(tmpl);
            setAllButtons(true);
        })
                .start();
    }

    private void enrollFromImage() {
        String name = nameField.getText().trim();
        if (name.isEmpty()) {
            JOptionPane.showMessageDialog(
                    this,
                    "Enter a user name first.",
                    "Error",
                    JOptionPane.ERROR_MESSAGE);
            return;
        }

        JFileChooser chooser = new JFileChooser();
        chooser.setDialogTitle("Select Fingerprint Image(s)");
        chooser.setMultiSelectionEnabled(true);
        chooser.setAcceptAllFileFilterUsed(true);
        chooser.addChoosableFileFilter(
                new FileNameExtensionFilter(
                        "Fingerprint Images (*.bmp, *.png, *.jpg, *.jpeg, *.tiff, *.tif, *.webp, *.wsq)",
                        "bmp",
                        "png",
                        "jpg",
                        "jpeg",
                        "tiff",
                        "tif",
                        "webp",
                        "wsq"));
        chooser.setFileFilter(chooser.getChoosableFileFilters()[1]);
        if (chooser.showOpenDialog(this) != JFileChooser.APPROVE_OPTION) {
            return;
        }

        File[] selected = chooser.getSelectedFiles();
        if (selected.length == 0 && chooser.getSelectedFile() != null) {
            selected = new File[] { chooser.getSelectedFile() };
        }
        if (selected.length == 0) {
            return;
        }

        List<File> files = filterSupportedImportFiles(selected);
        if (files.isEmpty()) {
            JOptionPane.showMessageDialog(
                    this,
                    "None of the selected files use a supported fingerprint image extension.",
                    "Unsupported Files",
                    JOptionPane.WARNING_MESSAGE);
            return;
        }

        setAllButtons(false);
        clearPreview();
        log("Enrolling " + files.size() + " image(s)…");
        status("Processing images…");

        new Thread(() -> {
            int success = 0;
            int failed = 0;
            for (File file : files) {
                String path = file.getAbsolutePath();
                log("Loading: " + file.getName());

                ZkfpLibrary.ZkfpTemplate tmpl = new ZkfpLibrary.ZkfpTemplate();
                PointerByReference b64Ref = new PointerByReference();

                int res = lib.zkfp_extract_from_image_file(path, tmpl, b64Ref);
                if (res == 1) {
                    handleB64(b64Ref);
                    enrollTemplate(name, "image-import", tmpl);
                    success++;
                } else {
                    log(
                            "  ✗ Failed: " +
                                    file.getName() +
                                    " — " +
                                    lib.zkfp_get_last_error());
                    failed++;
                }
                lib.zkfp_free_template(tmpl);
            }
            log(
                    "Enrollment complete: " +
                            success +
                            " OK, " +
                            failed +
                            " failed.");
            status(
                    "Enrolled " +
                            success +
                            " image(s)" +
                            (failed > 0 ? " (" + failed + " failed)" : ""));
            setAllButtons(true);
        })
                .start();
    }

    private List<File> filterSupportedImportFiles(File[] selectedFiles) {
        List<File> result = new ArrayList<>();
        for (File file : selectedFiles) {
            if (file == null || !file.isFile()) {
                continue;
            }
            String extension = getExtension(file.getName());
            if (extension != null && IMPORT_EXTENSIONS.contains(extension)) {
                result.add(file);
            } else {
                log("Skipping unsupported file: " + file.getName());
            }
        }
        return deduplicateFiles(result);
    }

    private List<File> deduplicateFiles(List<File> files) {
        Set<String> seen = new LinkedHashSet<>();
        List<File> unique = new ArrayList<>();
        for (File file : files) {
            String path = file.getAbsolutePath();
            if (seen.add(path)) {
                unique.add(file);
            }
        }
        return unique;
    }

    private void enrollTemplate(
            String name,
            String source,
            ZkfpLibrary.ZkfpTemplate tmpl) {
        byte[] iso = tmpl.getDataBytes();
        log(
                "Template ready — quality: " +
                        tmpl.quality +
                        " | size: " +
                        iso.length +
                        " bytes");

        int userId = db.addUser(name);
        db.saveTemplate(userId, source, iso);
        log(
                "'" +
                        name +
                        "' saved to PostgreSQL test backend (ID " +
                        userId +
                        ")");
        log("Run sync to bring the new enrollment into the local SDK DB.");

        status(
                "Enrolled to PostgreSQL test backend: " +
                        name +
                        " (ID " +
                        userId +
                        ")");
    }

    private void syncFromPostgres() {
        setAllButtons(false);
        status("Synchronizing local DB…");
        log("Running PostgreSQL → local SDK DB sync…");

        new Thread(() -> {
            try {
                rebuildLocalSnapshotWithMetrics("manual sync");
                status("Sync completed.");
            } catch (Exception e) {
                log("Sync failed: " + e.getMessage());
                status("Sync failed.");
            }
            setAllButtons(true);
        })
                .start();
    }

    private void rebuildLocalSnapshotWithMetrics(String reason) {
        long totalStart = System.nanoTime();

        long fetchStart = System.nanoTime();
        List<FingerprintRepository.UserRecord> users = db.loadAllUsers();
        List<FingerprintRepository.TemplateRecord> templates = db.loadAllTemplates();
        double fetchMs = (System.nanoTime() - fetchStart) / 1_000_000.0;

        log(
                "[sync] PostgreSQL fetch (" +
                        reason +
                        ") -> users=" +
                        users.size() +
                        ", templates=" +
                        templates.size() +
                        ", time=" +
                        String.format("%.1f ms", fetchMs));

        long rebuildStart = System.nanoTime();
        nativeDb.rebuildLocalSnapshot(LOCAL_DB_PATH, users, templates);
        double rebuildMs = (System.nanoTime() - rebuildStart) / 1_000_000.0;
        log(
                "[sync] Local snapshot rebuild -> time=" +
                        String.format("%.1f ms", rebuildMs));

        long ramStart = System.nanoTime();
        loadDatabaseToRAM();
        double ramMs = (System.nanoTime() - ramStart) / 1_000_000.0;
        log(
                "[sync] RAM gallery load -> time=" + String.format("%.1f ms", ramMs));

        double totalMs = (System.nanoTime() - totalStart) / 1_000_000.0;
        log(
                "[sync] Total synchronization time -> " +
                        String.format("%.1f ms", totalMs));
    }

    private void identify() {
        setAllButtons(false);
        clearPreview();
        log("Place your finger on the scanner to identify…");
        status("Waiting for finger…");

        new Thread(() -> {
            ZkfpLibrary.ZkfpTemplate tmpl = new ZkfpLibrary.ZkfpTemplate();
            PointerByReference b64Ref = new PointerByReference();

            int res = lib.zkfp_capture_full(tmpl, b64Ref);
            if (res == 1) {
                handleB64(b64Ref);

                byte[] probe = tmpl.getDataBytes();
                ZkfpLibrary.ZkfpIdentifyVerifyResult result = new ZkfpLibrary.ZkfpIdentifyVerifyResult();

                long t0 = System.nanoTime();
                int match = lib.zkfp_gallery_identify_with_verification(
                        probe,
                        probe.length,
                        result);
                double elapsedMs = (System.nanoTime() - t0) / 1_000_000.0;
                String timeStr = String.format("%.1fms", elapsedMs);

                if (match == 1) {
                    String userName = nativeDb.getUserNameById(result.user_id);
                    String displayName = (userName != null &&
                            !userName.isBlank())
                                    ? userName
                                    : ("ID " + result.user_id);
                    String msg = "MATCH — Name: " +
                            displayName +
                            " | Score: " +
                            result.verify_score +
                            " | Time: " +
                            timeStr;
                    log("--> " + msg);
                    status(msg);
                    JOptionPane.showMessageDialog(
                            this,
                            msg,
                            "Identified ✓",
                            JOptionPane.INFORMATION_MESSAGE);
                } else {
                    String msg = "No match. Name: -" +
                            " | Score: " +
                            result.verify_score +
                            " | Time: " +
                            timeStr;
                    log("--> " + msg);
                    status(msg);
                    JOptionPane.showMessageDialog(
                            this,
                            msg,
                            "Unknown ✗",
                            JOptionPane.WARNING_MESSAGE);
                }
            } else {
                log("Capture failed: " + lib.zkfp_get_last_error());
                status("Identification failed.");
            }
            lib.zkfp_free_template(tmpl);
            setAllButtons(true);
        })
                .start();
    }

    private void clearDatabase() {
        int answer = JOptionPane.showConfirmDialog(
                this,
                "Delete ALL fingerprints from PostgreSQL test data and clear local cache?",
                "Confirm Clear",
                JOptionPane.YES_NO_OPTION,
                JOptionPane.WARNING_MESSAGE);
        if (answer != JOptionPane.YES_OPTION) {
            return;
        }
        db.clearAll();
        lib.zkfp_gallery_clear();
        clearPreview();
        log(
                "PostgreSQL test backend cleared. Local SDK DB will be refreshed on next sync.");
        status("PostgreSQL test backend cleared.");
    }

    public static void main(String[] args) {
        try {
            FlatLightLaf.setup();
            UIManager.put("Button.arc", 14);
            UIManager.put("Component.arc", 14);
            UIManager.put("TextComponent.arc", 12);
            UIManager.put("ProgressBar.arc", 14);
            UIManager.put("Component.focusWidth", 1);
        } catch (Exception ignored) {
        }

        SwingUtilities.invokeLater(() -> new App().setVisible(true));
    }
}
