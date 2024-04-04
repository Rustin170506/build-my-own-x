package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_numDecodings(t *testing.T) {
	type args struct {
		s string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "case1",
			args: args{s: "12"},
			want: 2,
		},
		{
			name: "case2",
			args: args{s: "226"},
			want: 3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, numDecodings(tt.args.s), "numDecodings(%v)", tt.args.s)
		})
	}
}
