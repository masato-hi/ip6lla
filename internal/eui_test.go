package internal

import "testing"

func TestParseEUI48Address(t *testing.T) {
	validAddresses := []struct {
		input    string
		expected string
	}{
		{
			input:    "01:00:5E:90:10:FF",
			expected: "01-00-5E-90-10-FF",
		},
		{
			input:    "01:00:5e:90:10:ff",
			expected: "01-00-5E-90-10-FF",
		},
		{
			input:    "01-00-5E-90-10-FF",
			expected: "01-00-5E-90-10-FF",
		},
		{
			input:    "01-00-5e-90-10-ff",
			expected: "01-00-5E-90-10-FF",
		},
	}

	for _, tt := range validAddresses {
		got, err := ParseEUI48Address(tt.input)
		if err != nil {
			t.Fatalf("enxepected error %v", err)
		}

		if got.ToString() != tt.expected {
			t.Errorf("got %#v, want %#v", got, tt.expected)
		}
	}

	invalidAddresses := []struct {
		input   string
		wantErr string
	}{
		{
			input:   "",
			wantErr: "address string is empty",
		},
		{
			input:   "01",
			wantErr: "the delimiter is unknown",
		},
		{
			input:   "01:",
			wantErr: "each separated field must have 2 digits",
		},
		{
			input:   "01:2",
			wantErr: "each separated field must have 2 digits",
		},
		{
			input:   "01:2F",
			wantErr: "address string too short",
		},
		{
			input:   "01:2FA",
			wantErr: "each group must have 2 digits",
		},
		{
			input:   "01:2F-",
			wantErr: "unexpected character",
		},
		{
			input:   "01:0fg",
			wantErr: "unexpected character",
		},
		{
			input:   "01-00-5e-90-10-ff-ef",
			wantErr: "trailing garbage after address",
		},
	}

	for _, tt := range invalidAddresses {
		_, err := ParseEUI48Address(tt.input)
		if err == nil {
			t.Fatalf("wanted error %q; got none", tt.wantErr)
		}

		if err.Error() != tt.wantErr {
			t.Errorf("got error %#v, want error %#v", err.Error(), tt.wantErr)
		}
	}
}

func TestEUI48Address_Hyphenated(t *testing.T) {
	eui48, err := ParseEUI48Address("01:00:5E:90:10:FF")
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	got := eui48.Hyphenated()
	expected := "01-00-5E-90-10-FF"

	if got != expected {
		t.Errorf("got %#v, want %#v", got, expected)
	}
}

func TestEUI48Address_ColonSepareted(t *testing.T) {
	eui48, err := ParseEUI48Address("01-00-5E-90-10-FF")
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	got := eui48.ColonSepareted()
	expected := "01:00:5E:90:10:FF"

	if got != expected {
		t.Errorf("got %#v, want %#v", got, expected)
	}
}

func TestEUI48Address_ToEUI64Address(t *testing.T) {
	eui48, err := ParseEUI48Address("01-00-5E-90-10-FF")
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	eui64 := eui48.ToEUI64Address()

	got := eui64.ToString()
	expected := "01-00-5E-FF-FE-90-10-FF"

	if got != expected {
		t.Errorf("got %#v, want %#v", got, expected)
	}
}

func TestEUI48Address_ToIPv6Address(t *testing.T) {
	eui48, err := ParseEUI48Address("01:00:5E:90:10:FF")
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	ipv6, err := eui48.ToIPv6Address()
	if err != nil {
		t.Fatalf("unexpected error %#v", err)
	}

	got := ipv6.ToString()
	expected := "fe80::300:5eff:fe90:10ff"

	if got != expected {
		t.Errorf("got %#v, want %#v", got, expected)
	}
}
