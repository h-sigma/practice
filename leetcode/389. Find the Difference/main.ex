defmodule Solution do
  @spec find_the_difference(s :: String.t, t :: String.t) :: char
  def find_the_difference(s, t) do
    # Bless list subtraction
    String.graphemes(t) -- String.graphemes(s)
    |> hd
    |> :binary.at(0)
  end
end