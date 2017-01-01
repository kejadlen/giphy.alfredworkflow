require 'rake/clean'

require 'alphred/tasks'

desc 'Build for Travis'
task :travis do
  args = %w[ --standalone
             --path vendor/bundle
             --without development test ]
  sh "bundle install #{args.join(' ')}"
  sh "zip -rq #{ENV['TRAVIS_REPO_SLUG'].split(?/).last} *"
end

# task default: :imgpbcopy

# file imgpbcopy: 'imgpbcopy.swift' do |t|
#   input = t.prerequisites[0]
#   output = t.name
#   sh "xcrun -sdk macosx swiftc #{input} -o #{output}"
# end
# CLOBBER.include('imgpbcopy')

# task 'alfred:release' => :imgpbcopy
