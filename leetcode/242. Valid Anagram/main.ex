defmodule Solution do
  @spec repeated_substring_pattern(s :: String.t) :: boolean
  def repeated_substring_pattern(s) do
    len = String.length(s)
    (for i <- 0..len, 
        i < Integer.floor_div(len, 2), 
        rem(len, i + 1) == 0, 
        into: [],
        do: {String.slice(s, 0, i + 1), i + 1})
    |> Enum.any?(fn ({sub, sublen}) -> String.duplicate(sub, Integer.floor_div(len, sublen)) == s end)
  end
end