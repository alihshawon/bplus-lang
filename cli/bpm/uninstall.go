// cli/bpm/uninstall.go
package bpm

import (
    "errors"
    "fmt"
    "os"
    "path/filepath"
)

// ErrPackageNotInstalled is returned when the specified package is not installed.
var ErrPackageNotInstalled = errors.New("package not installed")

// UninstallPackage removes a package from the bpm extensions directory.
func UninstallPackage(pkgName string, global bool) error {
    installPath, err := getInstallPath(pkgName, global)
    if err != nil {
        return err
    }

    // Check if package directory exists
    if _, err := os.Stat(installPath); os.IsNotExist(err) {
        return ErrPackageNotInstalled
    }

    // Remove the package directory and its contents
    err = os.RemoveAll(installPath)
    if err != nil {
        return fmt.Errorf("failed to remove package %s: %w", pkgName, err)
    }

    fmt.Printf("Successfully uninstalled package: %s\n", pkgName)
    return nil
}

// getInstallPath returns the path where the package is installed.
func getInstallPath(pkgName string, global bool) (string, error) {
    var baseDir string
    if global {
        baseDir = GetGlobalExtensionsDir()
    } else {
        baseDir = GetLocalExtensionsDir()
    }
    if baseDir == "" {
        return "", errors.New("could not determine extensions directory")
    }
    return filepath.Join(baseDir, pkgName), nil
}
