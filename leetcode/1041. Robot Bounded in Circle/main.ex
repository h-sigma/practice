defmodule Solution do
  @spec is_robot_bounded(instructions :: String.t) :: boolean
  def is_robot_bounded(instructions) do
    {x, y, dir} = Enum.reduce(String.graphemes(instructions), {0, 0, :north}, &follow_instruction(&1, &2))
    cond do
        x == 0 and y == 0 -> true
        dir == :north -> false
        true -> true
    end
  end

  def follow_instruction("G", state), do: forward(state)
  def follow_instruction("L", state), do: left(state)
  def follow_instruction("R", state), do: right(state)

  def forward({x, y, dir}) do
    case dir do
        :north -> {x, y + 1, dir}
        :south -> {x, y - 1, dir}
        :east -> {x + 1, y, dir}
        :west -> {x - 1, y, dir}
    end
  end

  def left({x, y, dir}) do
    case dir do
        :north -> {x, y, :west}
        :south -> {x, y, :east}
        :east -> {x, y, :north}
        :west -> {x, y, :south}
    end
  end

  def right({x, y, dir}) do
    case dir do
        :north -> {x, y, :east}
        :south -> {x, y, :west}
        :east -> {x, y, :south}
        :west -> {x, y, :north}
    end
  end

end
