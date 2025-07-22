class Taskrush < Formula
  desc "A modern task runner with parallel execution and intelligent caching"
  homepage "https://github.com/iPeluwa/rush"
  url "https://github.com/iPeluwa/rush/releases/download/v0.2.5/rush-x86_64-apple-darwin.tar.gz"
  sha256 ""  # Will be filled automatically by the action
  license "MIT"

  def install
    bin.install "taskrush"
    bin.install "rush"
  end

  test do
    system "#{bin}/rush", "--version"
  end
end
