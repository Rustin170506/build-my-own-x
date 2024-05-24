package questions

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/hi-rustin/lg/src/utils"
)

func Test_mergeTwoLists(t *testing.T) {
	type args struct {
		list1 *utils.ListNode
		list2 *utils.ListNode
	}
	tests := []struct {
		name string
		args args
		want *utils.ListNode
	}{
		{
			name: "Example 1",
			args: args{
				list1: utils.BuildList([]int{1, 2, 4}),
				list2: utils.BuildList([]int{1, 3, 4}),
			},
			want: utils.BuildList([]int{1, 1, 2, 3, 4, 4}),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			require.Equal(t, utils.TraverseList(tt.want), utils.TraverseList(mergeTwoLists(tt.args.list1, tt.args.list2)))
		})
	}
}
