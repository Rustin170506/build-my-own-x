package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_kClosest(t *testing.T) {
	type args struct {
		points [][]int
		k      int
	}
	tests := []struct {
		name string
		args args
		want [][]int
	}{
		{
			name: "test 1",
			args: args{
				points: [][]int{{1, 3}, {-2, 2}},
				k:      1,
			},
			want: [][]int{{-2, 2}},
		},
	}

	for _, tt := range tests {
		require.Equal(t, tt.want, kClosest(tt.args.points, tt.args.k))
	}
}
