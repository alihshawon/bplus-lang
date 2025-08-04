// cli/bpm/disable.go
package bpm

import (
    "fmt"
)

// DisableExtension marks an installed extension as disabled in the config.
func DisableExtension(name string, global bool) error {
    config, err := LoadBPMConfig()
    if err != nil {
        return fmt.Errorf("failed to load bpm config: %w", err)
    }

    if _, exists := config.Extensions[name]; !exists {
        return fmt.Errorf("extension '%s' not found in config", name)
    }

    config.Extensions[name] = false
    if err := SaveBPMConfig(config); err != nil {
        return fmt.Errorf("failed to disable extension: %w", err)
    }

    fmt.Printf("Extension '%s' has been disabled.\n", name)
    return nil
}
