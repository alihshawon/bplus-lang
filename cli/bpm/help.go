// cli/bpm/help.go
package bpm

import (
    "fmt"
    "strings"

    "github.com/spf13/cobra"
)

var helpCmd = &cobra.Command{
    Use:   "help [command]",
    Short: "Show help for bpm or a specific command",
    Long:  `Displays help information about bpm commands and usage.`,
    Run: func(cmd *cobra.Command, args []string) {
        if len(args) == 0 {
            printGeneralHelp()
            return
        }

        commandName := args[0]
        c, _, err := cmd.Root().Find([]string{commandName})
        if err != nil || c == nil {
            fmt.Printf("Unknown command '%s'\n\n", commandName)
            printGeneralHelp()
            return
        }

        c.Help()
    },
}

func printGeneralHelp() {
    helpText := `
B+ Package Manager (bpm) - Commands Overview:

  init        Initialize a new B+ project
  install     Install a package or extension
  uninstall   Remove an installed package or extension
  update      Update a package or all packages
  publish     Publish your package to the registry
  search      Search for packages in the registry
  list        List installed packages/extensions
  enable      Enable an installed extension
  disable     Disable an installed extension
  config      Configure bpm settings
  doctor      Run diagnostics and fix issues
  verify      Verify integrity of a project
  info        Show details about a package
  help        Show help for bpm or a specific command

Use "bpm help [command]" for detailed help on a command.
`
    fmt.Println(strings.TrimSpace(helpText))
}

func init() {
    rootCmd.AddCommand(helpCmd)
}
