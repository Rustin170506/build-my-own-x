defmodule SolutionQ0145Test do
  use ExUnit.Case
  alias SolutionQ0145, as: S
  alias TreeNode

  test "postorder traversal" do
    assert S.postorder_traversal(nil) == []
    assert S.postorder_traversal(TreeNode.tree_from_array([1])) == [1]
    assert S.postorder_traversal(TreeNode.tree_from_array([1, 2])) == [2, 1]
    assert S.postorder_traversal(TreeNode.tree_from_array([1, 3, 2])) == [3, 2, 1]
  end
end
