// cli/bpm/init.go
package bpm

import (
    "fmt"
    "os"
    "path/filepath"
    "time"
)

const projectManifestTemplate = `[package]
name = "%s"
version = "0.1.0"
description = "A new B+ project"
authors = ["Your Name"]
license = "MIT"
created = "%s"

[dependencies]
`

// InitProject initializes a new B+ project in the specified directory.
// It creates a minimal bplus.toml manifest file and sets up folder structure.
func InitProject(path string, projectName string) error {
    // Create project directory if doesn't exist
    if _, err := os.Stat(path); os.IsNotExist(err) {
        if err := os.MkdirAll(path, 0755); err != nil {
            return fmt.Errorf("failed to create project directory: %w", err)
        }
    }

    manifestPath := filepath.Join(path, "bplus.toml")

    // Check if manifest already exists
    if _, err := os.Stat(manifestPath); err == nil {
        return fmt.Errorf("project manifest already exists at %s", manifestPath)
    }

    // Write manifest content
    manifestContent := fmt.Sprintf(projectManifestTemplate, projectName, time.Now().Format(time.RFC3339))
    if err := os.WriteFile(manifestPath, []byte(manifestContent), 0644); err != nil {
        return fmt.Errorf("failed to write manifest: %w", err)
    }

    fmt.Printf("Initialized new B+ project '%s' at %s\n", projectName, path)
    return nil
}
