defmodule SolutionQ0145 do
  @spec postorder_traversal(root :: TreeNode.t() | nil) :: [integer]
  def postorder_traversal(root) do
    case root do
      nil ->
        []

      %TreeNode{left: left, val: val, right: right} ->
        postorder_traversal(left) ++ postorder_traversal(right) ++ [val]
    end
  end
end
