defmodule NifLlvm2 do
  use Rustler, otp_app: :nif_llvm_2, crate: :llvm

  @moduledoc """
  Documentation for NifLlvm2.
  """


  @doc """
    ## Examples

    iex> NifLlvm2.generate_code()
    :ok
  """
  def generate_code(), do: exit(:nif_not_loaded)
end
