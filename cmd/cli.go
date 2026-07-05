package cmd

import (
	"os"

	"github.com/masato-hi/ip6ula/internal"
	"github.com/spf13/cobra"
)

func newRootCmd() cobra.Command {
	var colonSeparated bool
	var toUpperCase bool

	rootCmd := cobra.Command{
		Use:     "ip6ula",
		Short:   "Convert IPv6 unique-local addresses and MAC addresses each other.",
		Long:    `Convert modified EUI-64 based IPv6 unique-local addresses to MAC addresses each other.`,
		Example: "ip6ula -u -c fd00::300:5eff:fe90:10ff\nip6ula 01-00-5e-90-10-ff",
		Args:    cobra.MatchAll(cobra.ExactArgs(1)),
		RunE: func(cmd *cobra.Command, args []string) error {
			address := args[0]

			writer := cmd.OutOrStdout()
			return internal.Execute(writer, address, colonSeparated, toUpperCase)
		},
	}

	rootCmd.Flags().BoolVarP(&colonSeparated, "colon", "c", false, "display the MAC address in colon-separated")
	rootCmd.Flags().BoolVarP(&toUpperCase, "upcase", "u", false, "display the address in uppercase")

	return rootCmd
}

func Execute() {
	rootCmd := newRootCmd()
	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}
