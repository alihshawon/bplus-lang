// cli/bpm/enable.go
package bpm

import (
    "fmt"
    "os"
    "path/filepath"
)

// EnableExtension marks an installed extension as enabled by updating config.
func EnableExtension(name string, global bool) error {
    config, err := LoadBPMConfig()
    if err != nil {
        return fmt.Errorf("failed to load bpm config: %w", err)
    }

    extPath := filepath.Join(getExtensionsDir(global), name)
    if _, err := os.Stat(extPath); os.IsNotExist(err) {
        return fmt.Errorf("extension '%s' not installed", name)
    }

    config.Extensions[name] = true
    if err := SaveBPMConfig(config); err != nil {
        return fmt.Errorf("failed to enable extension: %w", err)
    }

    fmt.Printf("Extension '%s' has been enabled.\n", name)
    return nil
}
