defmodule Solution do
  @spec is_monotonic(nums :: [integer]) :: boolean
  def is_monotonic(nums) do
    Enum.chunk_every(nums, 2, 1, :discard)
    |> Enum.map(fn
        [a, b] when a < b -> -1
        [a, b] when a > b -> 1
        _ -> 0
    end)
    |> Enum.uniq()
    |> Enum.sort()
    |> IO.inspect()
    |> (fn list -> list != [-1, 1] and list != [-1, 0, 1] end).()
  end
end
