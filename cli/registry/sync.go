// registry/sync.go
package registry

import (
    "fmt"
    "net/http"
    "io"
    "os"
    "path/filepath"
)

// SyncPackage downloads and syncs a package from remote registry to local cache.
func SyncPackage(pkgName string, cacheDir string) error {
    url := fmt.Sprintf("https://registry.bpluslang.org/packages/%s/download", pkgName)

    resp, err := http.Get(url)
    if err != nil {
        return fmt.Errorf("failed to download package: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != 200 {
        return fmt.Errorf("failed to download package: status %s", resp.Status)
    }

    if err := os.MkdirAll(cacheDir, 0755); err != nil {
        return fmt.Errorf("failed to create cache directory: %w", err)
    }

    archivePath := filepath.Join(cacheDir, pkgName+".bpex")

    outFile, err := os.Create(archivePath)
    if err != nil {
        return fmt.Errorf("failed to create file: %w", err)
    }
    defer outFile.Close()

    _, err = io.Copy(outFile, resp.Body)
    if err != nil {
        return fmt.Errorf("failed to save package archive: %w", err)
    }

    return nil
}
