// cli/bpm/utils/filesystem.go
package bpm

import (
    "fmt"
    "io"
    "os"
    "path/filepath"
)

// CopyFile copies a single file from src to dst.
// Creates destination directories if needed.
func CopyFile(src, dst string) error {
    srcFile, err := os.Open(src)
    if err != nil {
        return fmt.Errorf("failed to open source file %s: %w", src, err)
    }
    defer srcFile.Close()

    // Ensure destination directory exists
    dstDir := filepath.Dir(dst)
    if err := os.MkdirAll(dstDir, 0755); err != nil {
        return fmt.Errorf("failed to create destination directory %s: %w", dstDir, err)
    }

    dstFile, err := os.Create(dst)
    if err != nil {
        return fmt.Errorf("failed to create destination file %s: %w", dst, err)
    }
    defer dstFile.Close()

    if _, err := io.Copy(dstFile, srcFile); err != nil {
        return fmt.Errorf("failed to copy data from %s to %s: %w", src, dst, err)
    }

    return nil
}

// CopyDir recursively copies a directory tree from src to dst.
// It skips symbolic links.
func CopyDir(src, dst string) error {
    entries, err := os.ReadDir(src)
    if err != nil {
        return fmt.Errorf("failed to read source directory %s: %w", src, err)
    }

    if err := os.MkdirAll(dst, 0755); err != nil {
        return fmt.Errorf("failed to create destination directory %s: %w", dst, err)
    }

    for _, entry := range entries {
        srcPath := filepath.Join(src, entry.Name())
        dstPath := filepath.Join(dst, entry.Name())

        info, err := entry.Info()
        if err != nil {
            return err
        }

        if info.Mode()&os.ModeSymlink != 0 {
            // Skip symlinks
            continue
        }

        if entry.IsDir() {
            if err := CopyDir(srcPath, dstPath); err != nil {
                return err
            }
        } else {
            if err := CopyFile(srcPath, dstPath); err != nil {
                return err
            }
        }
    }
    return nil
}

// RemoveDir deletes a directory and all its contents.
func RemoveDir(path string) error {
    return os.RemoveAll(path)
}
