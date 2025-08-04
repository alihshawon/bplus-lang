// cli/bpm/verify.go
package bpm

import (
    "errors"
    "fmt"
    "os"
    "path/filepath"
    "strings"
)

// VerifyProject checks the integrity of a B+ project directory.
func VerifyProject(path string) error {
    fmt.Println("Verifying B+ project integrity...")

    // Check if project directory exists
    if _, err := os.Stat(path); os.IsNotExist(err) {
        return fmt.Errorf("project directory does not exist: %s", path)
    }

    // Check if manifest file exists
    manifestPath := filepath.Join(path, "bplus.toml")
    if _, err := os.Stat(manifestPath); os.IsNotExist(err) {
        return errors.New("missing bplus.toml manifest file")
    }

    // Validate manifest file content
    data, err := os.ReadFile(manifestPath)
    if err != nil {
        return fmt.Errorf("failed to read manifest file: %w", err)
    }

    if !strings.Contains(string(data), "[package]") {
        return errors.New("invalid manifest: missing [package] section")
    }

    if !strings.Contains(string(data), "name") || !strings.Contains(string(data), "version") {
        return errors.New("invalid manifest: missing required fields")
    }

    fmt.Println("Project verified successfully.")
    return nil
}
