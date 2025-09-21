package internal

import (
	"bytes"
	"testing"
)

func TestExecute(t *testing.T) {
	tests := []struct {
		input          string
		colonSeparated bool
		toUpperCase    bool
		expected       string
	}{
		{
			input:          "01-00-5e-90-10-ff",
			colonSeparated: false,
			toUpperCase:    false,
			expected:       "fe80::300:5eff:fe90:10ff\n",
		},
		{
			input:          "01-00-5e-90-10-ff",
			colonSeparated: false,
			toUpperCase:    true,
			expected:       "FE80::300:5EFF:FE90:10FF\n",
		},
		{
			input:          "fe80::300:5eff:fe90:10ff",
			colonSeparated: false,
			toUpperCase:    false,
			expected:       "01-00-5e-90-10-ff\n",
		},
		{
			input:          "fe80::300:5eff:fe90:10ff",
			colonSeparated: true,
			toUpperCase:    false,
			expected:       "01:00:5e:90:10:ff\n",
		},
		{
			input:          "fe80::300:5eff:fe90:10ff",
			colonSeparated: true,
			toUpperCase:    true,
			expected:       "01:00:5E:90:10:FF\n",
		},
	}

	for _, tt := range tests {
		buf := bytes.Buffer{}
		if err := Execute(&buf, tt.input, tt.colonSeparated, tt.toUpperCase); err != nil {
			t.Fatalf("unexpected error %v", err)
		}

		got := buf.String()
		if got != tt.expected {
			t.Errorf("got %#v, want %#v", got, tt.expected)
		}
	}
}
