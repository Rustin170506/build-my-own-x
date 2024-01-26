defmodule Solution0094 do
  @spec inorder_traversal(root :: TreeNode.t() | nil) :: [integer]
  def inorder_traversal(root) do
    case root do
      nil ->
        []

      %TreeNode{left: left, val: val, right: right} ->
        inorder_traversal(left) ++ [val] ++ inorder_traversal(right)
    end
  end
end
