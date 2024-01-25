defmodule ListNode do
  @type t :: %__MODULE__{
          val: integer,
          next: ListNode.t() | nil
        }
  defstruct val: 0, next: nil

  # utility function to create a list from an array
  def list_from_array([]), do: nil

  def list_from_array([head | tail]) do
    %ListNode{val: head, next: list_from_array(tail)}
  end

  # utility function to create an array from a list
  def array_from_list(nil), do: []

  def array_from_list(%ListNode{val: val, next: next}) do
    [val | array_from_list(next)]
  end

  # utility function to print a list
  def print_list(nil), do: IO.puts("nil")

  def print_list(%ListNode{val: val, next: next}) do
    IO.puts(val)
    print_list(next)
  end
end
