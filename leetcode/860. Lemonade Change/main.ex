defmodule Solution do
  @spec lemonade_change(bills :: [integer]) :: boolean
  def lemonade_change(bills) do
    {cust, _, _} =
      Enum.reduce_while(bills, {0, 0, 0}, fn bill, acc = {cust, fives, tens} ->
        case bill do
          5 -> {:cont, {cust + 1, fives + 1, tens}}
          10 when fives > 0 -> {:cont, {cust + 1, fives - 1, tens + 1}}
          20 when fives > 0 and tens > 0 -> {:cont, {cust + 1, fives - 1, tens - 1}}
          20 when fives > 2 -> {:cont, {cust + 1, fives - 3, tens}}
          _ -> {:halt, acc}
        end
      end)

    cust == Enum.count(bills)
  end
end
