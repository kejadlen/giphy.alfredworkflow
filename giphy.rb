$LOAD_PATH.unshift(File.expand_path('../vendor/bundle', __FILE__))
require 'bundler/setup'
require 'json'

require 'alphred'
require 'faraday'

module Giphy
  API_KEY = 'dc6zaTOxFJmzC'

  class Gif
    attr_reader :data

    def initialize(data)
      @data = data
    end

    def thumbnail
      return @thumbnail if defined?(@thumbnail)

      url = data['images']['fixed_width_small_still']['url']
      @thumbnail = Thumbnail.new(id, url)
    end

    def id
      data['id']
    end

    def size
      data['images']['original']['size']
    end

    def name
      url.split(?/).last.sub(/\-[^-]+$/, '')
    end

    def url
      data['url']
    end

    def gif_url
      data['images']['original']['url']
    end

    def urls
      Hash[%w[ url mp4 webp ].map { |key|
        [key, data['images']['original'][key]]
      }]
    end
  end

  class Thumbnail
    attr_reader *%i[ id url ]

    def initialize(id, url)
      @id, @url = id, url
    end

    def download!
      return if File.exist?(path)

      File.write(path, Faraday.get(url).body, mode: ?w)
    end

    def path
      ext = File.extname(url)
      File.join(dir, "#{id}#{ext}")
    end

    def dir
      return @dir if defined?(@dir)

      dir = File.expand_path(ENV['alfred_workflow_cache'])
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
      '%.1f%s' % case size
                 when (0...1_000)
                   [size, nil]
                 when (1_000...1_000_000)
                   [size / 1_000.0, 'KB']
                 else
                   [size / 1_000_000.0, 'MB']
                 end
    end
  end
end

if __FILE__ == $0
  include Alphred
  include Giphy

  query = ARGV.shift

  resp = Faraday.get('http://api.giphy.com/v1/gifs/search',
                     { q: query,
                       limit: 9,
                       api_key: API_KEY })
  data = JSON.load(resp.body)['data']
  gifs = data.map {|gif| Gif.new(gif) }

  threads = gifs.map do |gif|
    Thread.new { gif.thumbnail.download! }
  end
  threads.each(&:join)

  items = gifs.map do |gif|
    Item.new(
      title: gif.name,
      subtitle: "#{gif.id} - #{FileSize.new(gif.size)}",
      arg: gif.gif_url,
      icon: gif.thumbnail.path,
      mods: {
        alt: {
          arg: '',
          subtitle: '',
        },
      }
    )
  end

  # items << Alphred::Item.new(
  #   title: '[Powered By Giphy]',
  #   icon: 'icon.png',
  # )

  puts Items[*items].to_json
end
