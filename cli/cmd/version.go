package cmd

import vinfo "github.com/alihshawon/bplus-lang/cli/internal/version"

import (
	"fmt"

	"github.com/spf13/cobra"
)

// Version of the B+ CLI


func init() {
	rootCmd.AddCommand(&cobra.Command{
		Use:   "version",
		Short: "Print the version of B+ CLI",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Printf("B+ CLI version: %s (commit %s, date %s)\n", vinfo.Version, vinfo.Commit, vinfo.Date)
		},
	})
}
