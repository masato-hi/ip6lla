package internal

import (
	"errors"
	"fmt"
	"math"
	"strings"
)

const (
	extensionIdentifierPrefixHigh uint8 = 0xff
	extensionIdentifierPrefixLow  uint8 = 0xfe
)

type EUI48Address struct {
	a byte
	b byte
	c byte
	d byte
	e byte
	f byte
}

func (e EUI48Address) ToString() string {
	return fmt.Sprintf("%02X-%02X-%02X-%02X-%02X-%02X", e.a, e.b, e.c, e.d, e.e, e.f)
}

func (e EUI48Address) Hyphenated() string {
	return e.ToString()
}

func (e EUI48Address) ColonSepareted() string {
	return strings.ReplaceAll(e.ToString(), "-", ":")
}

func (e EUI48Address) ToEUI64Address() EUI64Address {
	return EUI64Address{
		a: e.a,
		b: e.b,
		c: e.c,
		d: extensionIdentifierPrefixHigh,
		e: extensionIdentifierPrefixLow,
		f: e.d,
		g: e.e,
		h: e.f,
	}
}

func (e EUI48Address) ToIPv6Address() (IPv6Address, error) {
	eui64 := e.ToEUI64Address()
	return eui64.ToIPv6Address()
}

type EUI64Address struct {
	a byte
	b byte
	c byte
	d byte
	e byte
	f byte
	g byte
	h byte
}

func (e EUI64Address) ToString() string {
	return fmt.Sprintf("%02X-%02X-%02X-%02X-%02X-%02X-%02X-%02X", e.a, e.b, e.c, e.d, e.e, e.f, e.g, e.h)
}

func (e EUI64Address) ToIPv6Address() (IPv6Address, error) {
	slice := [16]byte{
		0xfe,
		0x80,
		0x00,
		0x00,
		0x00,
		0x00,
		0x00,
		0x00,
		e.a ^ 0x02,
		e.b,
		e.c,
		e.d,
		e.e,
		e.f,
		e.g,
		e.h,
	}

	return IPv6AddressFromSlice16(slice)
}

func ParseEUI48Address(input string) (EUI48Address, error) {
	fields, err := parseSeparated(input, 6)
	if err != nil {
		return EUI48Address{}, err
	}

	eui := EUI48Address{
		a: fields[0],
		b: fields[1],
		c: fields[2],
		d: fields[3],
		e: fields[4],
		f: fields[5],
	}

	return eui, nil
}

func parseSeparated(input string, count int) ([]byte, error) {
	if len(input) < 1 {
		return nil, errors.New("address string is empty")
	}

	s := input
	isUpCase := containsUpperCase(s)

	var delimiter byte
	if containsHyphen(s) {
		delimiter = '-'
	} else if containsColon(s) {
		delimiter = ':'
	} else {
		return nil, errors.New("the delimiter is unknown")
	}

	fields := make([]byte, count)

	i := 0
	for i < count {
		off := 0
		acc := uint16(0)
		for ; off < len(s); off++ {
			ch := s[off]
			if isNumeric(ch) {
				acc = (acc << 4) + uint16(ch-'0')
			} else if isUpperCase(ch) {
				if !isUpCase {
					return nil, errors.New("contains non-uppercase characters")
				}

				acc = (acc << 4) + uint16(ch-'A'+10)
			} else if isLowerCase(ch) {
				if isUpCase {
					return nil, errors.New("contains non-lowercase characters")
				}

				acc = (acc << 4) + uint16(ch-'a'+10)
			} else {
				break
			}

			if off > 1 {
				return nil, errors.New("each group must have 2 digits")
			}
			if acc > math.MaxUint8 {
				return nil, errors.New("field has value >=2^8")
			}
		}

		if off < 2 {
			return nil, errors.New("each separated field must have 2 digits")
		}

		fields[i] = byte(acc)
		i += 1

		s = s[off:]
		if len(s) == 0 {
			break
		}

		if s[0] != delimiter {
			return nil, errors.New("unexpected character")
		}

		s = s[1:]
	}

	if len(s) != 0 {
		return nil, errors.New("trailing garbage after address")
	}

	if i < count {
		return nil, errors.New("address string too short")
	}

	return fields, nil
}

func containsUpperCase(input string) bool {
	return contains(input, isUpperCase)
}

func containsHyphen(input string) bool {
	return contains(input, func(b byte) bool { return b == '-' })
}

func containsColon(input string) bool {
	return contains(input, func(b byte) bool { return b == ':' })
}

func contains(input string, fn func(byte) bool) bool {
	for i := range len(input) {
		ch := input[i]
		if fn(ch) {
			return true
		}
	}
	return false
}

func isNumeric(ch byte) bool {
	return '0' <= ch && ch <= '9'
}

func isUpperCase(ch byte) bool {
	return 'A' <= ch && ch <= 'F'
}

func isLowerCase(ch byte) bool {
	return 'a' <= ch && ch <= 'f'
}
