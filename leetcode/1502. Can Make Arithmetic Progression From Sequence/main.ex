defmodule Solution do
  @moduledoc """
  Checks for an arithmetic progression by sorting all elements (a valid AP must be in sorted order), grabbing each pair of elements, mapping them to their difference, and ensuring all subtractions lead to the same difference by filtering the list to unique values.
  """

  @spec can_make_arithmetic_progression(arr :: [integer]) :: boolean
  def can_make_arithmetic_progression(arr) do
    Enum.sort(arr)
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.map(fn ([a, b]) -> b - a end)
    |> Enum.uniq()
    |> Enum.count()
    == 1
  end
end