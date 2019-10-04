require 'helix_runtime/build_task'

HelixRuntime::BuildTask.new("math_rust")

task default: :build
