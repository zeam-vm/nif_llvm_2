defmodule NifLlvm2 do
  use Rustler, otp_app: :nif_llvm_2, crate: :llvm

  @moduledoc """
  Documentation for NifLlvm2.
  """

  @doc """

  ## Examples

    iex> NifLlvm2.add(1, 2)
    {:ok, 3}

  """
  def add(_a, _b), do: exit(:nif_not_loaded)
end
