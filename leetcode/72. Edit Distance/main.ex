defmodule Solution do
  @spec min_distance(word1 :: String.t(), word2 :: String.t()) :: integer
  def min_distance(word1, word2) do
    word1_len = String.length(word1)
    word2_len = String.length(word2)
    word1_tuple = String.graphemes(word1) |> List.to_tuple()
    word2_tuple = String.graphemes(word2) |> List.to_tuple()

    # initial case
    dp = Map.new(0..word2_len, fn j -> {j, j} end)

    dp =
      Enum.reduce(1..word1_len//1, dp, fn i, prev_row ->
        curr_row = %{0 => i}

        Enum.reduce(1..word2_len//1, curr_row, fn j, curr_row ->
          cost =
            if elem(word1_tuple, i - 1) == elem(word2_tuple, j - 1) do
              Map.get(prev_row, j - 1)
            else
              1 +
                min(
                  Map.get(prev_row, j),
                  min(
                    Map.get(prev_row, j - 1),
                    Map.get(curr_row, j - 1)
                  )
                )
            end

          Map.put(curr_row, j, cost)
        end)
      end)

    Map.get(dp, word2_len)
  end
end
