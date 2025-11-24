class MultiQuery < Formula
  version '0.0.7'
  desc "Multi-database query executor with unified JSON output"
  homepage "https://github.com/w00fmeow/multi-query"

  if OS.mac?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "0f886b5fdd56a55e3353c482b2e95e97ea8aabde6882f4aa7680ccb0c9a45844"
  elsif OS.linux?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "b4e0ebbfe77fae7bca9ef830772d7d6d7556203f49ac89cdc60b4f44e6096000"
  end

  conflicts_with "multi-query"

  def install
    bin.install "multi-query"
    man1.install "doc/multi-query.1"

    bash_completion.install "complete/multi-query.bash" => "multi-query"
    zsh_completion.install "complete/_multi-query"
    fish_completion.install "complete/multi-query.fish"
  end
end
