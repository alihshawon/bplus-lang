// cli/bpm/utils/manifest.go
package bpm

import (
    "errors"
    "fmt"
    "os"
    "path/filepath"
    "strings"

    "github.com/pelletier/go-toml"
)

// Manifest represents the structure of bplus.toml manifest file.
type Manifest struct {
    Package      PackageSection            `toml:"package"`
    Dependencies map[string]string         `toml:"dependencies,omitempty"`
}

type PackageSection struct {
    Name        string `toml:"name"`
    Version     string `toml:"version"`
    Description string `toml:"description,omitempty"`
    Authors     []string `toml:"authors,omitempty"`
    License     string `toml:"license,omitempty"`
    Created     string `toml:"created,omitempty"`
}

// LoadManifest loads and parses a bplus.toml manifest file from given path.
func LoadManifest(path string) (*Manifest, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, fmt.Errorf("failed to read manifest file: %w", err)
    }

    var manifest Manifest
    if err := toml.Unmarshal(data, &manifest); err != nil {
        return nil, fmt.Errorf("failed to parse manifest file: %w", err)
    }

    if strings.TrimSpace(manifest.Package.Name) == "" {
        return nil, errors.New("manifest package name is empty")
    }
    if strings.TrimSpace(manifest.Package.Version) == "" {
        return nil, errors.New("manifest package version is empty")
    }

    return &manifest, nil
}
