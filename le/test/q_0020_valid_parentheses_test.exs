defmodule Q0020ValidParenthesesTest do
  use ExUnit.Case
  alias Solution0020, as: S

  test "is_valid returns true for empty string" do
    assert S.is_valid("") == true
  end

  test "is_valid returns true for single-element string" do
    assert S.is_valid("(") == false
    assert S.is_valid(")") == false
    assert S.is_valid("[") == false
    assert S.is_valid("]") == false
    assert S.is_valid("{") == false
    assert S.is_valid("}") == false
  end

  test "is_valid returns true for multi-element string" do
    assert S.is_valid("()") == true
    assert S.is_valid("[]") == true
    assert S.is_valid("{}") == true
    assert S.is_valid("()[]{}") == true
    assert S.is_valid("([{}])") == true
  end
end
