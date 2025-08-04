// cli/bpm/utils/printer.go
package bpm

import (
    "fmt"
    "os"
)

// PrintInfo prints an informational message to stdout.
func PrintInfo(format string, a ...interface{}) {
    fmt.Printf("[INFO] "+format+"\n", a...)
}

// PrintWarning prints a warning message to stdout.
func PrintWarning(format string, a ...interface{}) {
    fmt.Printf("[WARN] "+format+"\n", a...)
}

// PrintError prints an error message to stderr.
func PrintError(format string, a ...interface{}) {
    fmt.Fprintf(os.Stderr, "[ERROR] "+format+"\n", a...)
}
