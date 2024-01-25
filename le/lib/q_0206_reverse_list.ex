defmodule Solution0206 do
  import ListNode
  @spec reverse_list(head :: ListNode.t() | nil) :: ListNode.t() | nil
  def reverse_list(head) do
    do_reverse(head, nil)
  end

  defp do_reverse(nil, prev), do: prev

  defp do_reverse(%ListNode{next: next} = head, prev) do
    new_head = %{head | next: prev}
    do_reverse(next, new_head)
  end
end
