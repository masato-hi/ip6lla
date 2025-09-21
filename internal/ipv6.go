package internal

import (
	"errors"
	"net/netip"
)

type IPv6Address struct {
	inner netip.Addr
}

func (a IPv6Address) ToString() string {
	return a.inner.String()
}

func (a IPv6Address) IsLinkLocalUnicast() bool {
	return a.inner.IsLinkLocalUnicast()
}

func (a IPv6Address) ToEUI48Address() (EUI48Address, error) {
	eui64, err := a.ToEUI64Address()
	if err != nil {
		return EUI48Address{}, err
	}

	if eui64.d != extensionIdentifierPrefixHigh {
		return EUI48Address{}, errors.New("is not EUI-64 based link-local unicast address")

	}

	if eui64.e != extensionIdentifierPrefixLow {
		return EUI48Address{}, errors.New("is not EUI-64 based link-local unicast address")
	}

	eui48 := EUI48Address{
		a: eui64.a,
		b: eui64.b,
		c: eui64.c,
		d: eui64.f,
		e: eui64.g,
		f: eui64.h,
	}

	return eui48, nil
}

func (a IPv6Address) ToEUI64Address() (EUI64Address, error) {
	if !a.IsLinkLocalUnicast() {
		return EUI64Address{}, errors.New("is not link-local unicast address")
	}

	slice := a.inner.As16()

	eui64 := EUI64Address{
		a: slice[8] ^ 0x02,
		b: slice[9],
		c: slice[10],
		d: slice[11],
		e: slice[12],
		f: slice[13],
		g: slice[14],
		h: slice[15],
	}

	return eui64, nil
}

func ParseIPv6Address(input string) (IPv6Address, error) {
	addr, err := netip.ParseAddr(input)
	if err != nil {
		return IPv6Address{}, err
	}

	if !addr.Is6() {
		return IPv6Address{}, errors.New("is not IPv6 address")
	}

	return IPv6Address{inner: addr}, nil
}

func IPv6AddressFromSlice16(slice [16]byte) (IPv6Address, error) {
	addr := netip.AddrFrom16(slice)

	if !addr.Is6() {
		return IPv6Address{}, errors.New("is not IPv6 address")
	}

	return IPv6Address{inner: addr}, nil
}
