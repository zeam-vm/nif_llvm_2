defmodule NifLlvm2.MixProject do
  use Mix.Project

  def project do
    [
      app: :nif_llvm_2,
      version: "0.1.0",
      elixir: "~> 1.7",
      compilers: [:rustler] ++ Mix.compilers,
      rustler_crates: rustler_crates(),
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  defp rustler_crates() do
    [llvm: [
      path: "native/llvm",
      mode: :release,
    ]]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.18.0"},
      {:ok,      "~> 2.0"}
    ]
  end
end
