package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_longestCommonSubsequence(t *testing.T) {
	type args struct {
		text1 string
		text2 string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "Example 1",
			args: args{"abcde", "ace"},
			want: 3,
		},
		{
			name: "Example 2",
			args: args{"abc", "abc"},
			want: 3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, longestCommonSubsequence(tt.args.text1, tt.args.text2))
			require.Equal(t, tt.want, longestCommonSubsequenceIteration(tt.args.text1, tt.args.text2))
		})
	}
}
