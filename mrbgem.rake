MRuby::Gem::Specification.new('mruby-hohho') do |spec|
  spec.license = 'MIT'
  spec.authors = 'Yusuke Sangenya'

  spec.linker.libraries << 'hohho_ext'
  spec.linker.library_paths << '../lib' # ./mruby から見た path
end
