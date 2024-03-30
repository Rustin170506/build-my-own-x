package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRob2(t *testing.T) {
	type args struct {
		nums []int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			"test-case-1",
			args{[]int{2, 3, 2}},
			3,
		},
		{
			"test-case-2",
			args{[]int{1, 2, 3, 1}},
			4,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, rob2(tt.args.nums), "rob2(%v)", tt.args.nums)
		})
	}
}
