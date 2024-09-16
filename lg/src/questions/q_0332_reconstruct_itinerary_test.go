package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_findItinerary(t *testing.T) {
	type args struct {
		tickets [][]string
	}
	tests := []struct {
		name string
		args args
		want []string
	}{
		{
			name: "test 1",
			args: args{
				tickets: [][]string{{"JFK", "SFO"}, {"JFK", "ATL"}, {"SFO", "ATL"}, {"ATL", "JFK"}, {"ATL", "SFO"}},
			},
			want: []string{"JFK", "ATL", "JFK", "SFO", "ATL", "SFO"},
		},
		{
			name: "test 2",
			args: args{
				tickets: [][]string{{"JFK", "KUL"}, {"JFK", "NRT"}, {"NRT", "JFK"}},
			},
			want: []string{"JFK", "NRT", "JFK", "KUL"},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := findItinerary(tt.args.tickets)
			require.Equal(t, tt.want, got)
		})
	}
}
