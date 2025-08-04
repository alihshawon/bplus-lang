// cli/bpm/list.go
package bpm

import (
    "fmt"
    "os"
    "path/filepath"
)

// ListPackages displays all installed packages in the local or global extensions folder.
func ListPackages(global bool) error {
    var baseDir string
    if global {
        baseDir = GetGlobalExtensionsDir()
    } else {
        baseDir = GetLocalExtensionsDir()
    }

    if baseDir == "" {
        return fmt.Errorf("extensions directory not configured")
    }

    entries, err := os.ReadDir(baseDir)
    if err != nil {
        return fmt.Errorf("failed to read extensions directory: %w", err)
    }

    if len(entries) == 0 {
        fmt.Println("No packages installed.")
        return nil
    }

    fmt.Println("Installed packages:")
    for _, entry := range entries {
        if entry.IsDir() {
            fmt.Printf(" - %s\n", entry.Name())
        }
    }

    return nil
}
