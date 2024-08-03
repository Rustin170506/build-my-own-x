package questions

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_findOrder(t *testing.T) {
	type args struct {
		numCourses    int
		prerequisites [][]int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{
			name: "test 1",
			args: args{
				numCourses: 4,
				prerequisites: [][]int{
					{1, 0},
					{2, 0},
					{3, 1},
					{3, 2},
				},
			},
			want: []int{0, 1, 2, 3},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assert.Equalf(t, tt.want, findOrder(tt.args.numCourses, tt.args.prerequisites), "findOrder(%v, %v)", tt.args.numCourses, tt.args.prerequisites)
		})
	}
}
