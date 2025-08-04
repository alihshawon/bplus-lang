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
		compilerPath := "compiler/target/release/bplus-compiler" // Adjust this path as needed

		// Check if compiler executable exists
		if _, err := os.Stat(compilerPath); os.IsNotExist(err) {
			fmt.Println("Compiler not found. Please build it first with 'cargo build --release --workspace'")
			return
		}

		if len(args) > 0 {
			// Run the compiler with the provided file as argument
			fmt.Printf("Running file: %s\n", args[0])
			c := exec.Command(compilerPath, "run", args[0])
			c.Stdin = os.Stdin
			c.Stdout = os.Stdout
			c.Stderr = os.Stderr
			if err := c.Run(); err != nil {
				fmt.Fprintf(os.Stderr, "Error running file: %v\n", err)
			}
		} else {
			// Launch REPL if no file argument provided
			fmt.Println("Launching B+ REPL...")
			c := exec.Command(compilerPath)
			c.Stdin = os.Stdin
			c.Stdout = os.Stdout
			c.Stderr = os.Stderr
			if err := c.Run(); err != nil {
				fmt.Fprintf(os.Stderr, "Error launching REPL: %v\n", err)
			}
		}
	},
}
