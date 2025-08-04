// cli/bpm/utils/crypto.go
package bpm

import (
    "crypto/sha256"
    "encoding/hex"
    "fmt"
    "io"
    "os"
)

// ComputeFileSHA256 computes the SHA-256 checksum of a file at the given path.
func ComputeFileSHA256(filePath string) (string, error) {
    file, err := os.Open(filePath)
    if err != nil {
        return "", fmt.Errorf("failed to open file %s: %w", filePath, err)
    }
    defer file.Close()

    hasher := sha256.New()
    if _, err := io.Copy(hasher, file); err != nil {
        return "", fmt.Errorf("failed to hash file %s: %w", filePath, err)
    }

    checksum := hasher.Sum(nil)
    return hex.EncodeToString(checksum), nil
}
