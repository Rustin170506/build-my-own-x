defmodule Solution0121 do
  @spec max_profit(prices :: [integer]) :: integer
  def max_profit(prices) do
    prices
    |> Enum.reduce({0, hd(prices)}, fn price, {max_profit, min_price} ->
      new_max_profit = max(max_profit, price - min_price)
      new_min_price = min(min_price, price)
      {new_max_profit, new_min_price}
    end)
    |> elem(0)
  end
end
