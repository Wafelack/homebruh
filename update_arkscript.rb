#!/usr/bin/env ruby
require 'json'
require 'net/http'

exit if !File::exists?("packages.json")

packages = JSON.parse(IO.read('packages.json'))

updated = false

packages.each do |p|
  if p["name"] == "arkscript"
    version = `curl https://httpd.12f.pl/arkscript-version`.strip
    break if p["version"] == version
    p["version"] = version
    updated = true
    break
  end
end

exit if updated == false

puts "[+] Found a new arkscript version"
File.open("packages.json", "w") do |f| 
  f.write(JSON.pretty_generate(packages))
end
puts "[+] Added new version to packages.json"

system("git", "add", ".", exception: true)
system("git", "commit", "-m", ":package: Automatic update of arkscript package", exception: true)
system("git push", exception: true)
puts "[+] Successfully deployed the changes"
