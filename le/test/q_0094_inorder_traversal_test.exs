defmodule Q0094InorderTraversalTest do
  use ExUnit.Case
  alias Solution0094, as: S
  alias TreeNode

  test "inorder traversal" do
    assert S.inorder_traversal(nil) == []
    assert S.inorder_traversal(TreeNode.tree_from_array([1])) == [1]
  end
end
