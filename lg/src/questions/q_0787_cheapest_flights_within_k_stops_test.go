package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_findCheapestPrice(t *testing.T) {
	type args struct {
		n       int
		flights [][]int
		src     int
		dst     int
		k       int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test 1",
			args: args{
				n:       3,
				flights: [][]int{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}},
				src:     0,
				dst:     2,
				k:       1,
			},
			want: 200,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := findCheapestPrice(tt.args.n, tt.args.flights, tt.args.src, tt.args.dst, tt.args.k)
			require.Equal(t, tt.want, got)
		})
	}
}
