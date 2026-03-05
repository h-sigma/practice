# Definition for singly-linked list.
#
# defmodule ListNode do
#   @type t :: %__MODULE__{
#           val: integer,
#           next: ListNode.t() | nil
#         }
#   defstruct val: 0, next: nil
# end

defmodule Solution do
  @spec add_two_numbers(l1 :: ListNode.t() | nil, l2 :: ListNode.t() | nil) :: ListNode.t() | nil
  def add_two_numbers(l1, l2) do
    add_recursive(l1, l2, 0)
  end

  def add_recursive(%ListNode{val: v1, next: n1}, %ListNode{val: v2, next: n2}, carry) do
    sum = v1 + v2 + carry
    %ListNode{val: rem(sum, 10), next: add_recursive(n1, n2, div(sum, 10))}
  end

  def add_recursive(%ListNode{val: v1, next: n1}, nil, carry) do
    sum = v1 + carry
    %ListNode{val: rem(sum, 10), next: add_recursive(n1, nil, div(sum, 10))}
  end

  def add_recursive(nil, %ListNode{val: v2, next: n2}, carry) do
    sum = v2 + carry
    %ListNode{val: rem(sum, 10), next: add_recursive(nil, n2, div(sum, 10))}
  end

  def add_recursive(nil, nil, 0), do: nil
  def add_recursive(nil, nil, carry), do: %ListNode{val: carry, next: nil}
end
