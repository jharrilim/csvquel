# frozen_string_literal: true

class Cql < Formula
  desc "Query CSVs with SQL"
  homepage "https://github.com/jharrilim/csvquel"
  version "0.1.0"
  license "MIT"

  head do
    url "https://github.com/jharrilim/csvquel.git", branch: "master"
    depends_on "rust" => :build
  end

  on_macos do
    if Hardware::CPU.intel?
      url ""
      sha256 ""
    else
      url ""
      sha256 ""
    end

  end

  # on_linux do
  #   if Hardware::CPU.intel?
  #     url "https://github.com/jharrilim/csvquel/releases/download/#{version}/csvquel-#{version}-x86_64-linux.tar.xz"
  #     sha256 ""
  #   end
  # end

  def install
    if build.head?
      system "cargo", "install", *std_cargo_args
      libexec.install "runtime"
      libexec.install "target/release/hx"
    else
      libexec.install Dir["*"]
    end
  end

  test do
    system bin / "cql", "--version"
  end
end