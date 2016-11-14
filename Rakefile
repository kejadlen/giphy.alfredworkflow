require 'rake/clean'

require 'alphred/tasks'

task default: :imgpbcopy

file imgpbcopy: 'imgpbcopy.swift' do |t|
  input = t.prerequisites[0]
  output = t.name
  sh "xcrun -sdk macosx swiftc #{input} -o #{output}"
end
CLOBBER.include('imgpbcopy')

task 'alfred:release' => :imgpbcopy
