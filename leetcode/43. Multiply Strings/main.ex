defmodule Solution do
  ## This is a naive implementation of the multiplication algorithm we learned in school.
  ## Might do karatsuba multiplication if I have time.

  def multiply(_, "0"), do: "0"
  def multiply("0", _), do: "0"

  @spec multiply(num1 :: String.t, num2 :: String.t) :: String.t
  def multiply(num1, num2) do
    num1 = to_reversed_indexed_list(num1)
    num2 = to_reversed_indexed_list(num2)

    raw_sums =
        for {vi, i} <- num1, {vj, j} <- num2, reduce: %{} do
            acc -> Map.update(acc, i + j, vi * vj, &(&1 + vi * vj))
        end

    max_pos = length(num1) + length(num2) - 1

    {_final_carry, reversed_list} = Enum.reduce(
        0..max_pos,
        {0, []},
        fn pos, {carry, acc} ->
            sum = Map.get(raw_sums, pos, 0) + carry
            {div(sum, 10), [rem(sum, 10) | acc]}
        end
    )

    reversed_list
        |> Enum.drop_while(&(&1 == 0))
        |> Enum.join()
  end

  defp to_reversed_indexed_list(num) do
    num
    |> String.to_charlist()
    |> Enum.map(&(&1 - ?0))
    |> Enum.reverse()
    |> Enum.with_index()
  end
end
