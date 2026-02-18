defmodule Solution do
  @spec average(salary :: [integer]) :: float
  def average(salary) do
    max = Enum.max(salary)
    min = Enum.min(salary)
    sum = Enum.sum(salary)
    count = Enum.count(salary)

    (sum - max - min) / (count - 2)
  end
end