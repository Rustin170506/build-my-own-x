package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_fib(t *testing.T) {
	type args struct {
		n int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "test with n = 2",
			args: args{n: 2},
			want: 1,
		},
		{
			name: "test with n = 3",
			args: args{n: 3},
			want: 2,
		},
		{
			name: "test with n = 4",
			args: args{n: 4},
			want: 3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, tt.want, fib(tt.args.n))
			require.Equal(t, tt.want, fibIteration(tt.args.n))
		})
	}
}
