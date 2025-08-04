// cli/bpm/update.go
package bpm

import (
    "fmt"
    "os"
)

// UpdatePackage updates a specific package or all packages if pkgName is empty.
func UpdatePackage(pkgName string, global bool) error {
    if pkgName == "" {
        fmt.Println("Updating all installed packages...")
        pkgs, err := ListInstalledPackages(global)
        if err != nil {
            return fmt.Errorf("failed to list installed packages: %w", err)
        }

        for _, p := range pkgs {
            if err := updateSinglePackage(p, global); err != nil {
                fmt.Printf("Failed to update %s: %v\n", p, err)
            } else {
                fmt.Printf("Updated %s successfully.\n", p)
            }
        }
        return nil
    }

    // Update single package
    return updateSinglePackage(pkgName, global)
}

// updateSinglePackage handles updating of one package.
func updateSinglePackage(pkgName string, global bool) error {
    fmt.Printf("Checking for updates to package: %s\n", pkgName)

    // TODO:
    // - Fetch latest version info from registry
    // - Compare with installed version
    // - Download and replace if newer version exists
    // - Handle dependencies, signatures, etc.

    fmt.Println("Update feature is not yet implemented.")
    return nil
}

// ListInstalledPackages lists all installed packages in extensions directory.
func ListInstalledPackages(global bool) ([]string, error) {
    baseDir := ""
    if global {
        baseDir = GetGlobalExtensionsDir()
    } else {
        baseDir = GetLocalExtensionsDir()
    }

    if baseDir == "" {
        return nil, fmt.Errorf("extensions directory not configured")
    }

    entries, err := os.ReadDir(baseDir)
    if err != nil {
        return nil, err
    }

    var pkgs []string
    for _, e := range entries {
        if e.IsDir() {
            pkgs = append(pkgs, e.Name())
        }
    }

    return pkgs, nil
}
