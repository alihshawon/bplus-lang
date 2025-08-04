// cli/bpm/utils/cache.go
package bpm

import (
    "fmt"
    "os"
    "path/filepath"
    "time"
)

// CacheDir returns the path to the bpm cache directory in user's home.
func CacheDir() (string, error) {
    homeDir, err := os.UserHomeDir()
    if err != nil {
        return "", fmt.Errorf("failed to get user home directory: %w", err)
    }
    cacheDir := filepath.Join(homeDir, ".bpm", "cache")
    return cacheDir, nil
}

// EnsureCacheDir makes sure the cache directory exists.
func EnsureCacheDir() (string, error) {
    cacheDir, err := CacheDir()
    if err != nil {
        return "", err
    }
    if err := os.MkdirAll(cacheDir, 0755); err != nil {
        return "", fmt.Errorf("failed to create cache directory: %w", err)
    }
    return cacheDir, nil
}

// IsCacheValid checks if cached file at path is still valid based on maxAge duration.
func IsCacheValid(path string, maxAge time.Duration) bool {
    info, err := os.Stat(path)
    if err != nil {
        return false
    }
    return time.Since(info.ModTime()) < maxAge
}
