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
	result := make([]string, 0)
	var dfs func(node *utils.TreeNode)
	dfs = func(node *utils.TreeNode) {
		if node == nil {
			result = append(result, "null")
			return
		}
		result = append(result, fmt.Sprintf("%v", node.Val))
		dfs(node.Left)
		dfs(node.Right)
	}
	dfs(root)
	return strings.Join(result, ",")
}

// Deserializes your encoded data to tree.
func (this *Codec) deserialize(data string) *utils.TreeNode {
	nodes := strings.Split(data, ",")
	i := 0
	var dfs func() *utils.TreeNode
	dfs = func() *utils.TreeNode {
		nodeVal := nodes[i]
		if nodeVal == "null" {
			i++
			return nil
		}
		val, _ := strconv.Atoi(nodeVal)
		node := &utils.TreeNode{Val: val}
		i++
		node.Left = dfs()
		node.Right = dfs()
		return node
	}

	return dfs()
}
