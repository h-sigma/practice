defmodule Solution do
  @spec two_sum(nums :: [integer], target :: integer) :: [integer]
  def two_sum(nums, target) do
    [ result | _] = for {a, aidx} <- 
        Enum.with_index(nums), 
        {b, bidx} <- Enum.slice(Enum.with_index(nums), (aidx+1)..Enum.count(nums)), 
        a + b == target, 
        aidx != bidx, 
        do: [aidx, bidx]
    result
  end
end