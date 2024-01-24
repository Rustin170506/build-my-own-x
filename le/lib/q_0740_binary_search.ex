defmodule Solution0740 do
  @spec search(nums :: [integer], target :: integer) :: integer
  def search(nums, target) do
    search(nums, target, 0, length(nums) - 1)
  end

  defp search(nums, target, l, r) do
    if l > r do
      -1
    else
      middle = div(l + r, 2)

      cond do
        Enum.at(nums, middle) < target ->
          search(nums, target, middle + 1, r)

        Enum.at(nums, middle) > target ->
          search(nums, target, l, middle - 1)

        Enum.at(nums, middle) == target ->
          middle
      end
    end
  end
end
