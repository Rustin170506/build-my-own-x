defmodule Solution do
  @spec two_sum(numbers :: [integer], target :: integer) :: [integer]
  def two_sum(numbers, target) do
    two_sum(numbers, target, 0, length(numbers) - 1)
  end

  defp two_sum(numbers, target, i, j) do
    case Enum.at(numbers, i) + Enum.at(numbers, j) do
      ^target ->
        [i + 1, j + 1]

      sum ->
        if sum > target do
          two_sum(numbers, target, i, j - 1)
        else
          two_sum(numbers, target, i + 1, j)
        end
    end
  end
end
