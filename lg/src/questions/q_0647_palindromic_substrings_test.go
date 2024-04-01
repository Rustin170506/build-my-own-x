package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_countSubstrings(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			"case 1",
			args{"abc"},
			3,
		},
		{
			"case 2",
			args{"aaa"},
			6,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, countSubstrings(tt.args.s), "countSubstrings(%v)", tt.args.s)
		})
	}
}
