MRUBY_CONFIG=File.expand_path(ENV["MRUBY_CONFIG"] || ".github_actions_build_config.rb")
MRUBY_VERSION=ENV["MRUBY_VERSION"] || "3.1.0"

file :mruby do
  sh "git clone --depth=1 https://github.com/mruby/mruby.git"
  if MRUBY_VERSION != 'master'
    Dir.chdir 'mruby' do
      sh "git fetch --tags"
      rev = %x{git rev-parse #{MRUBY_VERSION}}
      sh "git checkout #{rev}"
    end
  end
end

desc "touch mrb_hohho.c"
file 'src/mrb_hohho.c' => 'lib/*' do
  sh 'touch src/mrb_hohho.c'
end

desc "rust lib"
file 'lib/*' => 'ext/**/*' do
  sh "cd ext && cargo build --release && cp target/release/libhohho_ext.a ../lib"  
end

file 'ext/**/*'

desc "compile binary"
task :compile => ['mruby', 'src/mrb_hohho.c'] do
  sh "cd mruby && rake all MRUBY_CONFIG=#{MRUBY_CONFIG}"
end

desc "test"
task :test => 'compile' do
  sh "cd mruby && rake all test MRUBY_CONFIG=#{MRUBY_CONFIG}"
end

desc "cleanup"
task :clean do
  exit 0 unless File.directory?('mruby')
  sh "cd mruby && rake deep_clean"
end

task :default => :compile
