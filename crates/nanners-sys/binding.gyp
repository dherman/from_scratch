{
  "targets": [{
    "target_name": "nanners",
    "sources": [ "src/nanners.cc" ],
    "include_dirs": [ "<!(node -e \"require('nan')\")" ]
  }]
}
