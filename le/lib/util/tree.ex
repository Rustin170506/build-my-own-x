defmodule TreeNode do
  @type t :: %__MODULE__{
          val: integer,
          left: TreeNode.t() | nil,
          right: TreeNode.t() | nil
        }
  defstruct val: 0, left: nil, right: nil

  # Utility function to create a tree from a list of integers
  def tree_from_array([]), do: nil

  def tree_from_array(list) do
    tree_from_array(list, 1)
  end

  # Helper function to handle recursive tree building
  defp tree_from_array(list, index) when index > length(list), do: nil

  defp tree_from_array(list, index) do
    value = Enum.at(list, index - 1)

    %__MODULE__{
      val: value,
      left: tree_from_array(list, index * 2),
      right: tree_from_array(list, index * 2 + 1)
    }
  end

  # Utility function to create a list from a tree
  def array_from_tree(tree) do
    case tree do
      nil ->
        []

      %TreeNode{left: left, val: val, right: right} ->
        array_from_tree(left) ++ [val] ++ array_from_tree(right)
    end
  end

  # Utility function to print a tree
  def print_tree(nil), do: IO.puts("nil")

  def print_tree(%__MODULE__{val: val, left: left, right: right}) do
    IO.puts(val)
    print_tree(left)
    print_tree(right)
  end
end
