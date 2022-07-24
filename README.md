# mruby-hohho   [![build](https://github.com/genya0407/mruby-hohho/actions/workflows/ci.yml/badge.svg)](https://github.com/genya0407/mruby-hohho/actions/workflows/ci.yml)
Hohho class

## install by mrbgems
- add conf.gem line to `build_config.rb`

```ruby
MRuby::Build.new do |conf|

    # ... (snip) ...

    conf.gem :github => 'eva/mruby-hohho'
end
```
## example
```ruby
p Hohho.hi
#=> "hi!!"
t = Hohho.new "hello"
p t.hello
#=> "hello"
p t.bye
#=> "hello bye"
```

## License
under the MIT License:
- see LICENSE file
