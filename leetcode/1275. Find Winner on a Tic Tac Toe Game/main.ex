defmodule Solution do
  @spec tictactoe(moves :: [[integer]]) :: String.t()
  def tictactoe(moves) do
    grid =
      moves
      |> Enum.with_index()
      |> Enum.reduce(
        Tuple.duplicate(" ", 9),
        fn {[i, j], idx}, acc -> play(idx, i, j, acc) end
      )

    cond do
      is_winner?(grid, "X") -> "A"
      is_winner?(grid, "O") -> "B"
      has_space?(grid) -> "Pending"
      true -> "Draw"
    end
  end

  defp play(idx, i, j, grid), do: put_elem(grid, i * 3 + j, idx_to_char(idx))
  defp idx_to_char(idx), do: if(rem(idx, 2) == 0, do: "X", else: "O")

  defp is_winner?(grid, ch) do
    case grid do
      # Horizontal wins
      {^ch, ^ch, ^ch, _, _, _, _, _, _} -> true
      {_, _, _, ^ch, ^ch, ^ch, _, _, _} -> true
      {_, _, _, _, _, _, ^ch, ^ch, ^ch} -> true
      # Vertical wins
      {^ch, _, _, ^ch, _, _, ^ch, _, _} -> true
      {_, ^ch, _, _, ^ch, _, _, ^ch, _} -> true
      {_, _, ^ch, _, _, ^ch, _, _, ^ch} -> true
      # Diagonal wins
      {^ch, _, _, _, ^ch, _, _, _, ^ch} -> true
      {_, _, ^ch, _, ^ch, _, ^ch, _, _} -> true
      # No match
      _ -> false
    end
  end

  defp has_space?(grid) do
    Enum.any?(0..8, fn i -> elem(grid, i) == " " end)
  end
end
