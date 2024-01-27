defmodule Q0144PreorderTraversalTest do
  use ExUnit.Case
  alias SolutionQ0144, as: S
  alias TreeNode

  test "preorder traversal" do
    assert S.preorder_traversal(nil) == []
    assert S.preorder_traversal(TreeNode.tree_from_array([1])) == [1]
    assert S.preorder_traversal(TreeNode.tree_from_array([1, 2])) == [1, 2]
    assert S.preorder_traversal(TreeNode.tree_from_array([1, 3, 2])) == [1, 3, 2]
  end
end
