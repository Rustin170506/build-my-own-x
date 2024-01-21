defmodule SolutionTest do
  use ExUnit.Case
  alias Solution, as: S

  test "two_sum with target found" do
    assert S.two_sum([2, 7, 11, 15], 9) == [1, 2]
    assert S.two_sum([2, 3, 4], 6) == [1, 3]
    assert S.two_sum([0, 0, 3, 4], 0) == [1, 2]
  end
end
