package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_characterReplacement(t *testing.T) {
	type args struct {
		s string
		k int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "Example 1",
			args: args{
				s: "ABAB",
				k: 2,
			},
			want: 4,
		},
		{
			name: "Example 2",
			args: args{
				s: "AABABBA",
				k: 1,
			},
			want: 4,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, characterReplacement(tt.args.s, tt.args.k))
		})
	}
}
