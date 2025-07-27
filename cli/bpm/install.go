// cli/bpm/install.go

package bpm

import (
	"fmt"

	"github.com/spf13/cobra"
)

var InstallCmd = &cobra.Command{
	Use:   "install [package]",
	Short: "Install a B+ package (Not Implemented)",
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 {
			fmt.Println("Please specify a package to install.")
			return
		}
		fmt.Printf("Installing package %s... (Not Implemented)\n", args[0])
	},
}