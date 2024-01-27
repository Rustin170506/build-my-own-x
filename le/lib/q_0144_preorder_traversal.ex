defmodule SolutionQ0144 do
  @spec preorder_traversal(root :: TreeNode.t() | nil) :: [integer]
  def preorder_traversal(root) do
    case root do
      nil ->
        []

      %TreeNode{left: left, val: val, right: right} ->
        [val] ++ preorder_traversal(left) ++ preorder_traversal(right)
    end
  end
end
