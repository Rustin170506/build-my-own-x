package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestIsValid(t *testing.T) {
	tests := []struct {
		in  string
		out bool
	}{
		{"()", true},
		{"()[]{}", true},
		{"(]", false},
		{"([)]", false},
		{"{[]}", true},
		{"[", false},
		{"[[]", false},
		{"[[]]", true},
	}

	for _, test := range tests {
		require.Equal(t, test.out, isValid(test.in))
	}
}
