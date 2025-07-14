require "fileutils"

def run(cmd)
  puts "#{cmd}"
  system(cmd) or abort("#{cmd} failed!")
end

if ARGV.length < 1
  puts "Please provid subproject name."
end

sub = ARGV[0]

HOME = ENV["HOME"]
dest_dir = "#{HOME}/temp/rust/rust-learn/#{sub}"
FileUtils.mkdir_p(dest_dir)

files = Dir.glob("#{sub}/**", File::FNM_DOTMATCH).reject { |f| f =~ /\A\.\.?\z/ }

FileUtils.cp_r(files, dest_dir, remove_destination: true)

Dir.chdir(dest_dir) do
  run("chmod -R u+x .")
  run("cargo run")
end