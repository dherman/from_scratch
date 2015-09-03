{
  "targets": [{
    "target_name": "nanners",
    "sources": [ "src/nanners.cc" ],
    "include_dirs": [ "<!(node -e \"require('nan')\")" ]
  },{
    "target_name": "sizes",
    "sources": [ "src/sizes.cc" ],
    "include_dirs": [ "<!(node -e \"require('nan')\")" ]
  }]
}
