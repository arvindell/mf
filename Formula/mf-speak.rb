class MfSpeak < Formula
  desc "A spicy little command wrapper that provides audible feedback with attitude"
  homepage "https://github.com/arvindell/mf"
  url "https://github.com/arvindell/mf/archive/refs/tags/v0.3.2.tar.gz"
  sha256 "5a9ca3031ef9c8b7ec8b7711e5368f57031e02f8dfc5c32609f4f9508e6cb966"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
    # Symlink the binary to just "mf"
    bin.install_symlink "#{bin}/mf" => "mf"
  end

  test do
    assert_match "mf-speak", shell_output("#{bin}/mf --version")
  end
end 