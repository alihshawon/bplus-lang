// registry/publish_api.go
package registry

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
)

// PackageManifest represents the metadata sent to the registry when publishing.
type PackageManifest struct {
    Name        string `json:"name"`
    Version     string `json:"version"`
    Description string `json:"description,omitempty"`
    Author      string `json:"author,omitempty"`
    License     string `json:"license,omitempty"`
}

// PublishPackage sends package metadata and archive to remote registry API.
func PublishPackage(manifest PackageManifest, archiveData []byte, authToken string) error {
    url := "https://registry.bpluslang.org/api/v1/packages/publish"

    // Prepare multipart/form-data or JSON payload with metadata and archive bytes.
    // For simplicity, we'll assume JSON metadata and separate upload for archive in real implementation.

    jsonData, err := json.Marshal(manifest)
    if err != nil {
        return fmt.Errorf("failed to marshal manifest: %w", err)
    }

    req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
    if err != nil {
        return fmt.Errorf("failed to create request: %w", err)
    }
    req.Header.Set("Content-Type", "application/json")
    if authToken != "" {
        req.Header.Set("Authorization", "Bearer "+authToken)
    }

    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        return fmt.Errorf("failed to send request: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        return fmt.Errorf("publish failed: %s", resp.Status)
    }

    return nil
}
