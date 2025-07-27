// cli/cmd/run.go

package cmd

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

var runCmd = &cobra.Command{
	Use:   "run [file]",
	Short: "Run a B+ script file",
	Long:  `Compiles and runs a B+ script file. If no file is provided, it starts the REPL.`,
	Run: func(cmd *cobra.Command, args []string) {
		compilerPath := "compiler/target/release/bplus-compiler" // Adjust path as needed

		// Ensure the compiler is built
		if _, err := os.Stat(compilerPath); os.IsNotExist(err) {
			fmt.Println("Compiler not found. Please build it first with 'cargo build --release --workspace'")
			return
		}

		if len(args) > 0 {
			// Logic to run a file would go here
			fmt.Printf("Running file: %s\n", args[0])
			// Example: exec.Command(compilerPath, "run", args[0])
		} else {
			// Run the REPL
			fmt.Println("Launching B+ REPL...")
			c := exec.Command(compilerPath)
			c.Stdin = os.Stdin
			c.Stdout = os.Stdout
			c.Stderr = os.Stderr
			c.Run()
		}
	},
}