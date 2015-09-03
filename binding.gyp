{
  "targets": [{
    "target_name": "from_scratch",
    "sources": ["from_scratch.cc"],
    "include_dirs": ["<!(node -e \"require('nan')\")"],
    "libraries": ["../target/release/libfrom_scratch.dylib"]
  }]
}
