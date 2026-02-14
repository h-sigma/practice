defmodule Solution do
  @spec maximum_wealth(accounts :: [[integer]]) :: integer
  def maximum_wealth(accounts) do
    for i <- accounts, reduce: 0 do
        acc -> max(acc, Enum.sum(i))
    end

    # alternatively, I also like this one:
    # accounts |> Enum.map(&Enum.sum/1) |> Enum.max()
  end
end
