package bpm

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "bpm",
	Short: "B+ Package Manager (bpm) manages B+ language packages and extensions",
	Long:  `bpm is a full-featured package manager for the B+ programming language.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Welcome to bpm! Use --help to see available commands.")
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}
}

// GetRootCmd returns the bpm root command (useful if integrating as subcommand)
func GetRootCmd() *cobra.Command {
	return rootCmd
}

func init() {
	rootCmd.AddCommand(initCmd)
	rootCmd.AddCommand(installCmd)
	rootCmd.AddCommand(uninstallCmd)
	rootCmd.AddCommand(updateCmd)
	rootCmd.AddCommand(publishCmd)
	rootCmd.AddCommand(searchCmd)
	rootCmd.AddCommand(listCmd)
}
