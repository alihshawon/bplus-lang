// cli/bpm/config.go
package bpm

import (
    "encoding/json"
    "errors"
    "fmt"
    "os"
    "path/filepath"
)

const bpmConfigFile = ".bpmconfig"

type BPMConfig struct {
    Extensions map[string]bool `json:"extensions"`
}

// getConfigPath returns the path to the bpm config file.
func getConfigPath(global bool) (string, error) {
    if global {
        homeDir, err := os.UserHomeDir()
        if err != nil {
            return "", fmt.Errorf("failed to get home directory: %w", err)
        }
        return filepath.Join(homeDir, bpmConfigFile), nil
    }
    return filepath.Join(".", bpmConfigFile), nil
}

// LoadBPMConfig loads the bpm config from file.
func LoadBPMConfig() (*BPMConfig, error) {
    return LoadBPMConfigFrom(false)
}

// LoadBPMConfigFrom loads the bpm config from a given scope (local/global).
func LoadBPMConfigFrom(global bool) (*BPMConfig, error) {
    configPath, err := getConfigPath(global)
    if err != nil {
        return nil, err
    }

    if _, err := os.Stat(configPath); os.IsNotExist(err) {
        return &BPMConfig{Extensions: map[string]bool{}}, nil
    }

    data, err := os.ReadFile(configPath)
    if err != nil {
        return nil, fmt.Errorf("failed to read config file: %w", err)
    }

    var config BPMConfig
    if err := json.Unmarshal(data, &config); err != nil {
        return nil, fmt.Errorf("failed to parse config file: %w", err)
    }

    if config.Extensions == nil {
        config.Extensions = map[string]bool{}
    }

    return &config, nil
}

// SaveBPMConfig saves the config to the default local config file.
func SaveBPMConfig(config *BPMConfig) error {
    return SaveBPMConfigTo(config, false)
}

// SaveBPMConfigTo saves the bpm config to a given scope (local/global).
func SaveBPMConfigTo(config *BPMConfig, global bool) error {
    configPath, err := getConfigPath(global)
    if err != nil {
        return err
    }

    data, err := json.MarshalIndent(config, "", "  ")
    if err != nil {
        return fmt.Errorf("failed to encode config: %w", err)
    }

    if err := os.WriteFile(configPath, data, 0644); err != nil {
        return fmt.Errorf("failed to write config file: %w", err)
    }

    return nil
}

// IsExtensionEnabled checks if an extension is enabled.
func IsExtensionEnabled(name string) (bool, error) {
    config, err := LoadBPMConfig()
    if err != nil {
        return false, err
    }

    enabled, exists := config.Extensions[name]
    if !exists {
        return false, errors.New("extension not found in config")
    }

    return enabled, nil
}
