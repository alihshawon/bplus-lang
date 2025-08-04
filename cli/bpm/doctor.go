// cli/bpm/doctor.go
package bpm

import (
    "fmt"
    "os"
    "runtime"
    "time"
)

// RunDoctor runs diagnostic checks to verify bpm and system health.
func RunDoctor() {
    fmt.Println("Running bpm diagnostics...\n")

    checkOS()
    checkHomeDir()
    checkNetwork()
    checkCache()
    checkManifestTemplate()

    fmt.Println("\n All checks completed.")
}

func checkOS() {
    fmt.Println("• Operating System:")
    fmt.Printf("  OS: %s\n  ARCH: %s\n  Go Version: %s\n", runtime.GOOS, runtime.GOARCH, runtime.Version())
}

func checkHomeDir() {
    fmt.Println("\n• Home Directory:")
    if home, err := os.UserHomeDir(); err == nil {
        fmt.Printf("  %s\n", home)
    } else {
        fmt.Printf("  Failed to detect home directory: %v\n", err)
    }
}

func checkNetwork() {
    fmt.Println("\n• Network Check:")
    // Placeholder: Add ping or HTTP check to registry URL if needed
    fmt.Println("  (network check skipped — implement actual ping if needed)")
}

func checkCache() {
    fmt.Println("\n• Cache Directory:")
    cacheDir, err := os.UserCacheDir()
    if err != nil {
        fmt.Printf("  Failed to find cache directory: %v\n", err)
    } else {
        fmt.Printf("  %s\n", cacheDir)
    }
}

func checkManifestTemplate() {
    fmt.Println("\n• Template Rendering Test:")
    result := fmt.Sprintf(projectManifestTemplate, "test_project", time.Now().Format(time.RFC3339))
    if len(result) > 0 {
        fmt.Println("  Manifest template OK")
    } else {
        fmt.Println("  Manifest template rendering failed")
    }
}
