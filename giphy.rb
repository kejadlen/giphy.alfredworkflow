$LOAD_PATH.unshift(File.expand_path("../vendor/bundle", __FILE__))
require "bundler/setup"
require "json"

require "alphred"
require "faraday"

module Giphy
  API_KEY = "dc6zaTOxFJmzC"

  class Gif
    attr_reader :data

    def initialize(data)
      @data = data
    end

    def thumbnail
      return @thumbnail if defined?(@thumbnail)

      url = self.data["images"]["fixed_width_small_still"]["url"]
      @thumbnail = Thumbnail.new(self.name, url)
    end

    def id
      self.data["id"]
    end

    def size
      self.data["images"]["original"]["size"]
    end

    def name
      self.data["url"].split(?/).last.sub(/\-[^-]+$/, "")
    end

    def urls
      Hash[%w[ url mp4 webp ].map {|key| [key, self.data["images"]["original"][key]] }]
    end
  end

  class Thumbnail
    attr_reader *%i[ name url ]

    def initialize(name, url)
      @name, @url = name, url
    end

    def download!
      File.write(self.path, Faraday.get(url).body, mode: ?w)
    end

    def path
      ext = File.extname(self.url)
      File.join(self.dir, "#{self.name}#{ext}")
    end

    def dir
      return @dir if defined?(@dir)

      dir = File.expand_path(ENV["alfred_workflow_cache"])
      Dir.mkdir(dir) unless Dir.exist?(dir)
      @dir = dir
    end
  end

  class FileSize
    attr_reader :size

    def initialize(size)
      @size = size.to_i
    end

    def to_s
      "%.1f%s" % case self.size
                 when (0...1_000)
                   [self.size, nil]
                 when (1_000...1_000_000)
                   [self.size / 1_000.0, "KB"]
                 else
                   [self.size / 1_000_000.0, "MB"]
                 end
    end
  end
end

if __FILE__ == $0
  query = ARGV.shift

  resp = Faraday.get("http://api.giphy.com/v1/gifs/search",
                     { q: query,
                       limit: 9,
                       api_key: Giphy::API_KEY })
  data = JSON.load(resp.body)["data"]
  gifs = data.map {|gif| Giphy::Gif.new(gif) }

  threads = gifs.map do |gif|
    Thread.new do
      gif.thumbnail.download!
    end
  end

  threads.each(&:join)

  items = gifs.map do |gif|
    Alphred::Item.new(
      title: gif.name,
      subtitle: "#{gif.id} - #{Giphy::FileSize.new(gif.size)}",
      arg: JSON.dump(gif.urls),
      icon: gif.thumbnail.path,
    )
  end

  # items << Alphred::Item.new(
  #   title: "[Powered By Giphy]",
  #   icon: "icon.png",
  # )

  puts Alphred::Items.new(*items).to_xml
end
