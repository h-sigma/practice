defmodule Solution do
  @spec check_straight_line(coordinates :: [[integer]]) :: boolean
  def check_straight_line(coordinates) do
    case coordinates do
      [] ->
        false

      [_] ->
        false

      [first, second | rest] ->
        first_slope = slope([first, second])

        [second | rest]
        |> Enum.chunk_every(2, 1, :discard)
        |> Enum.map(&slope(&1))
        |> Enum.all?(&is_approx(&1, first_slope, 0.0001))
    end
  end

  defp slope([[x1 | _], [x2 | _]]) when x1 == x2, do: :infinity
  defp slope([[x1, y1], [x2, y2]]), do: (y2 - y1) / (x2 - x1)

  defp is_approx(a, b, eps) when a == :infinity or b == :infinity, do: a === b
  defp is_approx(a, b, eps), do: abs(a - b) < eps
end
