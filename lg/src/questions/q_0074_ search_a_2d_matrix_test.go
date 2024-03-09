package questions

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestSearchMatrix(t *testing.T) {
	tests := []struct {
		matrix [][]int
		target int
		want   bool
	}{
		{
			matrix: [][]int{
				{1, 3, 5, 7},
				{10, 11, 16, 20},
				{23, 30, 34, 60},
			},
			target: 3,
			want:   true,
		},
		{
			matrix: [][]int{
				{1, 3, 5, 7},
				{10, 11, 16, 20},
				{23, 30, 34, 60},
			},
			target: 13,
			want:   false,
		},
		{
			matrix: [][]int{
				{1, 3, 5, 7},
				{10, 11, 16, 20},
				{23, 30, 34, 60},
			},
			target: 60,
			want:   true,
		},
		{
			matrix: [][]int{
				{1, 3, 5, 7},
				{10, 11, 16, 20},
				{23, 30, 34, 60},
			},
			target: 0,
			want:   false,
		},
		{
			matrix: [][]int{
				{1},
			},
			target: 1,
			want:   true,
		},
		{
			matrix: [][]int{
				{1},
			},
			target: 2,
			want:   false,
		},
		{
			matrix: [][]int{
				{1, 3},
			},
			target: 3,
			want:   true,
		},
		{
			matrix: [][]int{
				{1, 1},
			},
			target: 3,
			want:   false,
		},
	}
	for _, tt := range tests {
		t.Run("test", func(t *testing.T) {
			got := searchMatrix(tt.matrix, tt.target)
			require.Equal(t, tt.want, got)
		})
	}
}
