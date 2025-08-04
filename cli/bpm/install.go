// cli/bpm/install.go
package bpm

import (
    "errors"
    "fmt"
    "io"
    "net/http"
    "os"
    "path/filepath"
)

// ErrPackageNotFound is returned when a package is not found in the registry or local path.
var ErrPackageNotFound = errors.New("package not found")

// InstallPackage installs a package by name or local path into the project or global extensions folder.
func InstallPackage(pkgNameOrPath string, global bool) error {
    if isLocalPath(pkgNameOrPath) {
        fmt.Printf("Installing package from local path: %s\n", pkgNameOrPath)
        return installFromLocal(pkgNameOrPath, global)
    } else {
        fmt.Printf("Installing package from remote registry: %s\n", pkgNameOrPath)
        return installFromRemote(pkgNameOrPath, global)
    }
}

func isLocalPath(input string) bool {
    // crude check for local path (starts with ./, ../ or /)
    if len(input) > 0 && (input[0] == '.' || input[0] == '/' || input[0] == '\\') {
        return true
    }
    return false
}

func installFromLocal(path string, global bool) error {
    // Here, we would copy package files from local path to global or local bpm directory
    // For simplicity, just print message
    fmt.Printf("Copying package files from %s...\n", path)
    // TODO: Implement copy logic, dependency checks, manifest validation
    return nil
}

func installFromRemote(pkgName string, global bool) error {
    // This is a stub: real implementation would query registry API, download package archive,
    // verify checksum/signature, and extract files to bpm directory.

    // Example: GET https://registry.bpluslang.org/packages/{pkgName}/download

    url := fmt.Sprintf("https://registry.bpluslang.org/packages/%s/download", pkgName)

    resp, err := http.Get(url)
    if err != nil {
        return fmt.Errorf("failed to download package: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != 200 {
        return ErrPackageNotFound
    }

    // Save package archive to temp folder (stub)
    tmpDir := filepath.Join(os.TempDir(), "bpm", "downloads")
    if err := os.MkdirAll(tmpDir, 0755); err != nil {
        return fmt.Errorf("failed to create temp dir: %w", err)
    }

    archivePath := filepath.Join(tmpDir, pkgName+".bpex") // assuming .bpex package extension

    outFile, err := os.Create(archivePath)
    if err != nil {
        return fmt.Errorf("failed to create archive file: %w", err)
    }
    defer outFile.Close()

    _, err = io.Copy(outFile, resp.Body)
    if err != nil {
        return fmt.Errorf("failed to save package archive: %w", err)
    }

    fmt.Printf("Downloaded package archive to %s\n", archivePath)

    // TODO: Extract archive, verify checksum/signature, move to bpm extensions folder

    return nil
}
