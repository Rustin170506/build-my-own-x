defmodule Q0740BinarySearchTest do
  use ExUnit.Case
  doctest Solution0740

  test "search" do
    assert Solution0740.search([1, 2, 3, 4, 5], 3) == 2
    assert Solution0740.search([1, 2, 3, 4, 5], 5) == 4
    assert Solution0740.search([1, 2, 3, 4, 5], 1) == 0
    assert Solution0740.search([1, 2, 3, 4, 5], 6) == -1
  end
end
