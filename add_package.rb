#!/usr/bin/env ruby
require 'json'

if !File::exist?("packages.json")
    puts "Cannot find `packages.json` in the current folder"
    exit
end

packages = JSON.parse(IO.read('packages.json'))

name = ""
version = ""
description = ""
source = ""
authors = []


while true
    print "Enter package name: "
    name = gets.strip
    print "Enter package version: "
    version = gets.strip
    print "Enter brief package description: "
    description = gets.strip
    print "Write package link (remember: all packages have to be a single .tar.gz file): "
    source = gets.strip
    print "Write package authors (separated by a comma): "
    authors = gets.strip.split ','

    puts "\n    Summary\n==============="

    puts "name: " + name
    puts "version: " + version
    puts "description: " + description
    puts "source link: " + source
    puts "authors: " + authors.join(", ")

    print "\nIs this information correct ? [y/N]: "

    correct = gets
    if correct.strip.upcase == "Y"
        break
    end
end

packages.push(
    {
        "name" => name,
        "version" => version,
        "authors" => authors,
        "description" => description,
        "source" => source
    }
)

File.open("packages.json", "w") do |f| 
    
    f.write(JSON.pretty_generate(packages))
end

puts "[+] Successfully added package `" + name + "`"
