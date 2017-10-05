require 'bundler/setup'
require 'helix_runtime/build_task'
require 'rspec/core/rake_task'

HelixRuntime::BuildTask.new

RSpec::Core::RakeTask.new(:spec) do |t|
  t.verbose = false
end

task :spec => :build
task :default => :spec
