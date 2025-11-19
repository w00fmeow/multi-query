class MultiQuery < Formula
  version '0.0.6'
  desc "Multi-database query executor with unified JSON output"
  homepage "https://github.com/w00fmeow/multi-query"

  if OS.mac?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "b61c8680b611fbf9cb7b5bf5bc67e1a5814cdf8daa490fb4d7ef988e69c4ed0c"
  elsif OS.linux?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "e916a841e317b8ea3238d3409479970d35ac69d658e0325e136384fd50f591dd"
  end

  conflicts_with "multi-query"

  def install
    bin.install "multi-query"
    man1.install "doc/multi-query.1"

    bash_completion.install "complete/multi-query.bash"
    zsh_completion.install "complete/_multi-query"
  end
end
