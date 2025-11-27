class MultiQuery < Formula
  version '0.0.8'
  desc "Multi-database query executor with unified JSON output"
  homepage "https://github.com/w00fmeow/multi-query"

  if OS.mac?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "61c84e89f37010bb8a2ac2f8032e76bea58287dc1cd0d9b9c50f9b391ef3e7b2"
  elsif OS.linux?
      url "https://github.com/w00fmeow/multi-query/releases/download/#{version}/multi-query-#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "29d7b9bf0f4267b6d97a0d6a35ec36289b382dfd9c2eb431dd3cf15347f7e14f"
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
