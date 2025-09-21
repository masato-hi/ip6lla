package internal

import "testing"

func TestIPv6Address_ToEUI64Address(t *testing.T) {
	ipv6, err := ParseIPv6Address("fe80::300:5eff:fe90:10ff")
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	eui64, err := ipv6.ToEUI64Address()
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	got := eui64.ToString()
	expected := "01-00-5E-FF-FE-90-10-FF"

	if got != expected {
		t.Errorf("got %#v, want %#v", got, expected)
	}
}

func TestIPv6Address_ToEUI48Address(t *testing.T) {
	ipv6, err := ParseIPv6Address("fe80::300:5eff:fe90:10ff")
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	eui48, err := ipv6.ToEUI48Address()
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	got := eui48.ToString()
	expected := "01-00-5E-90-10-FF"

	if got != expected {
		t.Errorf("got %#v, want %#v", got, expected)
	}
}
