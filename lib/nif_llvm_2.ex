defmodule NifLlvm2 do
  use Rustler, otp_app: :nif_llvm_2, crate: :llvm
  use OK.Pipe

  @moduledoc """
  Documentation for NifLlvm2.
  """

  @does_support_native "SYSTEM_ELIXIR_DOES_SUPPORT_NATIVE"

  def init() do
    case {initialize_native_target(), initialize_native_asm_printer()} do
      {:ok, :ok} ->
        System.put_env(@does_support_native, "true")
        {:ok, true}
      _ ->
        System.put_env(@does_support_native, "false")
        IO.puts "Target platform doesn't support native code."
        {:ok, false}
    end
  end

  def does_support_native() do
    case System.get_env(@does_support_native) do
      nil ->
        init()
        does_support_native()
      "true" -> true
      _ -> false
    end
  end

  def run_code() do
    case does_support_native() do
      true ->
        generate_code_nif()
        ~> execute_code_nif()
      _ ->
        {:error, "This platform doesn't support native."}
    end
  end

  def generate_code() do
    case does_support_native() do
      true ->
        generate_code_nif()
      _ ->
        {:error, "This platform doesn't support native."}
    end
  end

  def execute_code(code) do
    case does_support_native() do
      true ->
        execute_code_nif(code)
      _ ->
        {:error, "This platform doesn't support native."}
    end
  end

  defp generate_code_nif(), do: exit(:nif_not_loaded)

  defp execute_code_nif(_code), do: exit(:nif_not_loaded)

  defp initialize_native_target(), do: exit(:nif_not_loaded)

  defp initialize_native_asm_printer(), do: exit(:nif_not_loaded)

end
