project(
  'system-manager',
  'cpp',
  version: '0.1.0',
  default_options: [
    'cpp_std=c++23',
    'warning_level=3',
  ],
)

gtkmm = dependency('gtkmm-4.0')

executable(
  'system-manager',
  'src/ui/ui.cpp',
  'src/ui/ui.hpp',
  'src/main.cpp',
'src/daemons/progress.cpp',
'src/daemons/progress.hpp',
  dependencies: gtkmm,
  install: true,
)