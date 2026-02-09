defmodule Solution do
  @spec judge_circle(moves :: String.t) :: boolean
  def judge_circle(moves) do
    Enum.reduce(String.graphemes(moves), {0, 0}, &move(&1, &2)) == {0, 0}
  end

  def move("R", {x, y}), do: {x+1, y}
  def move("L", {x, y}), do: {x-1, y}
  def move("U", {x, y}), do: {x, y+1}
  def move("D", {x, y}), do: {x, y-1}
end
