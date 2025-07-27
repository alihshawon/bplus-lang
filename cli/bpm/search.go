// cli/bpm/search.go

package bpm

import (
	"fmt"

	"github.com/spf13/cobra"
)

var SearchCmd = &cobra.Command{
	Use:   "search [query]",
	Short: "Search for a B+ package (Not Implemented)",
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 {
			fmt.Println("Please provide a search query.")
			return
		}
		fmt.Printf("Searching for '%s'... (Not Implemented)\n", args[0])
	},
}