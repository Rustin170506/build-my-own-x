package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_longestPalindrome(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		{
			"old",
			args{"babad"},
			"bab",
		},
		{
			"even",
			args{"cbbd"},
			"bb",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, longestPalindrome(tt.args.s), "longestPalindrome(%v)", tt.args.s)
		})
	}
}
