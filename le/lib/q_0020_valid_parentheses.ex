defmodule Solution0020 do
  @spec is_valid(s :: String.t()) :: boolean
  def is_valid(s) do
    s
    |> String.codepoints()
    |> Enum.reduce({[], true}, fn char, {stack, valid} ->
      if valid do
        case char do
          "(" ->
            {["(" | stack], true}

          "[" ->
            {["[" | stack], true}

          "{" ->
            {["{" | stack], true}

          ")" ->
            case stack do
              ["(" | rest] -> {rest, true}
              _ -> {stack, false}
            end

          "]" ->
            case stack do
              ["[" | rest] -> {rest, true}
              _ -> {stack, false}
            end

          "}" ->
            case stack do
              ["{" | rest] -> {rest, true}
              _ -> {stack, false}
            end
        end
      else
        {stack, false}
      end
    end)
    |> case do
      {[], true} -> true
      _ -> false
    end
  end
end
