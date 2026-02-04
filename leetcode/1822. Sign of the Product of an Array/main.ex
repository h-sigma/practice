defmodule Solution do
  @moduledoc """
    Uses list comprehension to reduce the input into the product of each component of `nums`, and then retrieves its sign.
    We do not need to first map each component to its sign because Elixir does not have a limit on the integer size.
  """
  
  @spec array_sign(nums :: [integer]) :: integer
  def array_sign(nums) do
    for x <- nums, reduce: 1 do
        acc ->
            x * acc
    end
    |> sign_func()
  end

  def sign_func(x) when x < 0, do: -1
  def sign_func(x) when x > 0, do: 1
  def sign_func(0), do: 0
end