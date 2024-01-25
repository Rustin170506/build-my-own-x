defmodule Q0206ReverseListTest do
  use ExUnit.Case
  doctest Solution0206

  alias ListNode
  alias Solution0206

  test "reverse_list/1" do
    assert Solution0206.reverse_list(nil) == nil

    assert Solution0206.reverse_list(ListNode.list_from_array([1])) ==
             ListNode.list_from_array([1])

    assert Solution0206.reverse_list(ListNode.list_from_array([1, 2])) ==
             ListNode.list_from_array([2, 1])

    assert Solution0206.reverse_list(ListNode.list_from_array([1, 2, 3])) ==
             ListNode.list_from_array([3, 2, 1])

    assert Solution0206.reverse_list(ListNode.list_from_array([1, 2, 3, 4])) ==
             ListNode.list_from_array([4, 3, 2, 1])

    assert Solution0206.reverse_list(ListNode.list_from_array([1, 2, 3, 4, 5])) ==
             ListNode.list_from_array([5, 4, 3, 2, 1])
  end
end
