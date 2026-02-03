defmodule Solution do
  @spec is_anagram(s :: String.t, t :: String.t) :: boolean
  def is_anagram(s, t) do
    # Just sort the graphemes and see if they result in the same string, voila.
    Enum.sort(String.graphemes(s)) == Enum.sort(String.graphemes(t))
  end
end