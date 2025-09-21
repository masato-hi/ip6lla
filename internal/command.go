package internal

import (
	"fmt"
	"io"
	"strings"
)

func Execute(writer io.Writer, address string, colonSeparated bool, toUpperCase bool) error {
	if ipv6, err := ParseIPv6Address(address); err == nil {
		return convertIPv6ToEUI48(writer, ipv6, colonSeparated, toUpperCase)
	} else if eui48, err := ParseEUI48Address(address); err == nil {
		return convertEUI48ToIPv6(writer, eui48, toUpperCase)
	} else {
		return fmt.Errorf("is not IPv6 or MAC Address. %v", err)
	}
}

func convertIPv6ToEUI48(writer io.Writer, ipv6 IPv6Address, colonSeparated bool, toUpperCase bool) error {
	eui48, err := ipv6.ToEUI48Address()
	if err != nil {
		return err
	}

	var output string
	if colonSeparated {
		output = eui48.ColonSepareted()
	} else {
		output = eui48.Hyphenated()
	}

	if toUpperCase {
		output = strings.ToUpper(output)
	} else {
		output = strings.ToLower(output)
	}

	output = fmt.Sprintln(output)

	if _, err := io.WriteString(writer, output); err != nil {
		return err
	}

	return nil
}

func convertEUI48ToIPv6(writer io.Writer, eui48 EUI48Address, toUpperCase bool) error {
	ipv6, err := eui48.ToIPv6Address()
	if err != nil {
		return err
	}

	output := ipv6.ToString()

	if toUpperCase {
		output = strings.ToUpper(output)
	} else {
		output = strings.ToLower(output)
	}

	output = fmt.Sprintln(output)

	if _, err := io.WriteString(writer, output); err != nil {
		return err
	}

	return nil

}
