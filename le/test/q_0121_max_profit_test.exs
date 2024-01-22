defmodule Solution0121Test do
  use ExUnit.Case
  alias Solution0121, as: S

  test "max_profit returns 0 for single-element list" do
    assert S.max_profit([1]) == 0
  end

  test "max_profit returns correct profit for multi-element list" do
    assert S.max_profit([7, 1, 5, 3, 6, 4]) == 5
    assert S.max_profit([7, 6, 4, 3, 1]) == 0
  end
end
