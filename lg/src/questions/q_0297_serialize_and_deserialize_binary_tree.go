package questions

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/hi-rustin/lg/src/utils"
)

type Codec struct {
}

func CodecConstructor() Codec {
	return Codec{}
}

// Serializes a tree to a single string.
func (this *Codec) serialize(root *utils.TreeNode) string {
	if root == nil {
		return ""
	}

	strs := make([]string, 0)
	var dfs func(root *utils.TreeNode)
	dfs = func(root *utils.TreeNode) {
		if root == nil {
			strs = append(strs, "null")
			return
		}
		strs = append(strs, fmt.Sprintf("%d", root.Val))
		dfs(root.Left)
		dfs(root.Right)
	}

	dfs(root)
	return strings.Join(strs, ",")
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *utils.TreeNode {
	if data == "" {
		return nil
	}
	strs := strings.Split(data, ",")
	i := 0
	var dfs func() *utils.TreeNode
	dfs = func() *utils.TreeNode {
		str := strs[i]
		if str == "null" {
			i++
			return nil
		}
		node := &utils.TreeNode{}
		val, _ := strconv.Atoi(str)
		node.Val = val
		i++
		node.Left = dfs()
		node.Right = dfs()

		return node
	}

	return dfs()
}
