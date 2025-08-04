// cli/bpm/publish.go
package bpm

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
    "os"
    "path/filepath"
)

// PackageManifest represents metadata about the package to publish.
type PackageManifest struct {
    Name        string `json:"name"`
    Version     string `json:"version"`
    Description string `json:"description,omitempty"`
    Author      string `json:"author,omitempty"`
    License     string `json:"license,omitempty"`
}

// PublishPackage publishes the package located at projectPath to the remote registry.
func PublishPackage(projectPath string) error {
    manifestPath := filepath.Join(projectPath, "bplus.toml")
    if _, err := os.Stat(manifestPath); os.IsNotExist(err) {
        return fmt.Errorf("no bplus.toml manifest found in %s", projectPath)
    }

    manifest, err := LoadManifest(manifestPath)
    if err != nil {
        return fmt.Errorf("failed to load manifest: %w", err)
    }

    pkgMeta := PackageManifest{
        Name:        manifest.Name,
        Version:     manifest.Version,
        Description: manifest.Description,
        Author:      manifest.Author,
        License:     manifest.License,
    }

    jsonData, err := json.Marshal(pkgMeta)
    if err != nil {
        return fmt.Errorf("failed to marshal package manifest: %w", err)
    }

    // Example: POST to registry API endpoint
    url := "https://registry.bpluslang.org/api/v1/packages/publish"

    req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
    if err != nil {
        return fmt.Errorf("failed to create HTTP request: %w", err)
    }
    req.Header.Set("Content-Type", "application/json")

    // TODO: Add authentication headers if needed

    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        return fmt.Errorf("failed to send publish request: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != 200 {
        return fmt.Errorf("publish failed with status: %s", resp.Status)
    }

    fmt.Printf("Package %s@%s published successfully!\n", pkgMeta.Name, pkgMeta.Version)
    return nil
}
