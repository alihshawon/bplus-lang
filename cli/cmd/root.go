// cli/cmd/root.go

package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

// rootCmd is the base command for the B+ CLI tool.
var rootCmd = &cobra.Command{
	Use:   "bplus",
	Short: "B+ is a programming language with Bangla-inspired keywords.",
	Long: `B+ is an educational programming language designed to be easy to learn
for native Bengali speakers. This CLI tool allows you to run and compile B+ code.`,
}

// Execute runs the root command and handles errors.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Whoops. There was an error while executing your CLI: %s\n", err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(compileCmd)
}
