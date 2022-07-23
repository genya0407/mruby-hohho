##
## Hohho Test
##

assert("Hohho#hello") do
  t = Hohho.new "hello"
  assert_equal("hello", t.hello)
end

assert("Hohho#bye") do
  t = Hohho.new "hello"
  assert_equal("hello bye", t.bye)
end

assert("Hohho.hi") do
  assert_equal("hi!!", Hohho.hi)
end
