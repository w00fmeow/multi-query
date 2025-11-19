class MultiQuery < Formula
  version '0.0.6'
  desc "Multi-database query executor with unified JSON output"
  homepage "https://github.com/w00fmeow/multi-query"

  if OS.mac?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "e4a5232f0b5382eb0c3556f2637a8262b00ba4854fc451953e16a46eba83959c"
  elsif OS.linux?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "aee0821bc3fcc4b6260658ba1526afb1a23f79ea606e5c2c596c9385749bf3db"
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
