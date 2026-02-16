defmodule Solution do
  @spec spiral_order(matrix :: [[integer]]) :: [integer]
  def spiral_order(matrix) do
    rows = Enum.count(matrix)
    cols = Enum.count(hd(matrix))
    total_elements = rows * cols

    # Initial state
    initial_acc = %{radial: 0, at: 1, dir: :right}

    # Iterate from 1 to the total number of elements
    {result, _final_state} = Enum.map_reduce(1..total_elements, initial_acc, fn _, acc ->
      # Calculate the next state based on current values
      next_state = case acc.dir do
        :right when rem(acc.at + acc.radial, cols) == 0 ->
          %{acc | at: acc.at + cols, dir: :down}

        :right ->
          %{acc | at: acc.at + 1}

        :left when rem(acc.at - acc.radial + cols - 1, cols) == 0 ->
          %{acc | at: acc.at - cols, dir: :up}

        :left ->
          %{acc | at: acc.at - 1}

        :down when acc.at == (cols) * (rows - acc.radial) - acc.radial ->
          %{acc | at: acc.at - 1, dir: :left}

        :down ->
          %{acc | at: acc.at + cols}

        :up when acc.at == (acc.radial + 1) + (acc.radial + 1) * cols ->
          %{acc | radial: acc.radial + 1, at: acc.at + 1, dir: :right}

        :up ->
          %{acc | at: acc.at - cols}
      end

      {acc.at, next_state}
    end)

    Enum.map(result, fn index ->
      r = div(index - 1, cols)
      c = rem(index - 1, cols)
      matrix |> Enum.at(r) |> Enum.at(c)
    end)
  end
end
