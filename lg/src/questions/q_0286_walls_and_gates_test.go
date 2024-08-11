package questions

import (
	testing "testing"
)

func TestWallsAndGates(t *testing.T) {
	type args struct {
		rooms [][]int
	}
	tests := []struct {
		name string
		args args
		want [][]int
	}{
		{
			name: "test case 1",
			args: args{
				rooms: [][]int{
					{2147483647, -1, 0, 2147483647},
					{2147483647, 2147483647, 2147483647, -1},
					{2147483647, -1, 2147483647, -1},
					{0, -1, 2147483647, 2147483647},
				},
			},
			want: [][]int{
				{3, -1, 0, 1},
				{2, 2, 1, -1},
				{1, -1, 2, -1},
				{0, -1, 3, 4},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			WallsAndGates(tt.args.rooms)
			compare2DArray := func(a, b [][]int) bool {
				if len(a) != len(b) {
					return false
				}
				for i := 0; i < len(a); i++ {
					if len(a[i]) != len(b[i]) {
						return false
					}
					for j := 0; j < len(a[i]); j++ {
						if a[i][j] != b[i][j] {
							return false
						}
					}
				}
				return true
			}
			if !compare2DArray(tt.args.rooms, tt.want) {
				t.Errorf("WallsAndGates() = %v, want %v", tt.args.rooms, tt.want)
			}
		})
	}
}
