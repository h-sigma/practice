defmodule Solution do
  @spec length_of_last_word(s :: String.t) :: integer
  def length_of_last_word(s) do
    String.split(s, " ")
    |> Enum.filter(&(String.length(&1) > 0))
    |> Enum.at(-1)
    |> String.length()
  end
end
