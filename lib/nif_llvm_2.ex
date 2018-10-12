defmodule NifLlvm2 do
  use Rustler, otp_app: :nif_llvm_2, crate: :llvm
  use OK.Pipe

  @moduledoc """
  Documentation for NifLlvm2.
  """


  def run_code() do
  	generate_code()
  	~> execute_code()
  end

  def generate_code(), do: exit(:nif_not_loaded)

  def execute_code(_code), do: exit(:nif_not_loaded)
end
