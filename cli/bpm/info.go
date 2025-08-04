// cli/bpm/info.go
package bpm

import (
    "fmt"
    "os"
    "path/filepath"
)

// InfoPackage prints detailed information about an installed package.
func InfoPackage(pkgName string, global bool) error {
    installPath, err := getInstallPath(pkgName, global)
    if err != nil {
        return err
    }

    manifestPath := filepath.Join(installPath, "bplus.toml")
    if _, err := os.Stat(manifestPath); os.IsNotExist(err) {
        return fmt.Errorf("package manifest not found for %s", pkgName)
    }

    manifest, err := LoadManifest(manifestPath)
    if err != nil {
        return fmt.Errorf("failed to load manifest for %s: %w", pkgName, err)
    }

    fmt.Printf("Package: %s\n", manifest.Name)
    fmt.Printf("Version: %s\n", manifest.Version)
    fmt.Printf("Description: %s\n", manifest.Description)
    fmt.Printf("Author: %s\n", manifest.Author)
    fmt.Printf("License: %s\n", manifest.License)
    fmt.Printf("Path: %s\n", installPath)

    return nil
}
