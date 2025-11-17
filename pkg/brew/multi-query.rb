class MultiQuery < Formula
  version ''
  desc "Multi-database query executor with unified JSON output"
  homepage "https://github.com/w00fmeow/multi-query"

  if OS.mac?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.linux?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 ""
  end

  conflicts_with "multi-query"

  def install
    bin.install "multi-query"
    man1.install "doc/multi-query.1"

    bash_completion.install "complete/multi-query.bash"
    zsh_completion.install "complete/_multi-query"
  end
end
