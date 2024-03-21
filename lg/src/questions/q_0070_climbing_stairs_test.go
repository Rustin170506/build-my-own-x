package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestCliimbingStairs(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want int
	}{
		{
			name: "testcase 1",
			n:    2,
			want: 2,
		},
		{
			name: "testcase 2",
			n:    3,
			want: 3,
		},
		{
			name: "testcase 3",
			n:    4,
			want: 5,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := climbStairs(tt.n)
			require.Equal(t, tt.want, got)
		})
	}
}
