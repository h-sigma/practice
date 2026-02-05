defmodule Solution do
  @moduledoc """
  Uses pattern matching and tail recursive calls to solve.
  """

  @spec roman_to_int(s :: String.t) :: integer
  def roman_to_int(s) do
    left_parse(s)
  end

  def left_parse("IV" <> rest), do: 4 + left_parse(rest)
  def left_parse("IX" <> rest), do: 9 + left_parse(rest)
  def left_parse("XL" <> rest), do: 40 + left_parse(rest)
  def left_parse("XC" <> rest), do: 90 + left_parse(rest)
  def left_parse("CD" <> rest), do: 400 + left_parse(rest)
  def left_parse("CM" <> rest), do: 900 + left_parse(rest)

  def left_parse("I" <> rest), do: 1 + left_parse(rest)
  def left_parse("V" <> rest), do: 5 + left_parse(rest)
  def left_parse("X" <> rest), do: 10 + left_parse(rest)
  def left_parse("L" <> rest), do: 50 + left_parse(rest)
  def left_parse("C" <> rest), do: 100 + left_parse(rest)
  def left_parse("D" <> rest), do: 500 + left_parse(rest)
  def left_parse("M" <> rest), do: 1000 + left_parse(rest)
  def left_parse(""), do: 0
end
