# nestor

Static asset management using object storage

## Plan

- [ ] Download all static assets from obj store using the public facing URL
- [ ] Identify currently referenced assets from the markdown files
    - [ ] Parse all md files in the specified directories
    - [ ] Find all asset tags in the md files (currently, `![]()` tags), use regex to obtain paths to actual assets
- [ ] Obtain final list of assets to maintain in obj store
    - [ ] Hash file contents for specified file types, (.png, .jpg,etc) 
 (reason behind looking for specific file types is to obtain assets which might be interleaved with *.md, if the assets are held in a separate folder, should be a hash calc on all files in the specific folder only)
    - [ ] Compare with the list of assets referenced in the md files, remove any entries present only in calculated hashes (deleted assets)
- [ ] Modify files on obj store
    - [ ] Parse an `assets-lock.json` and diff the file paths and hashes with the updated in-memory list
    - [ ] Prepare the list of assets to be pushed, updated and deleted (does not require calls to the obj store)
    - [ ] Update assets on obj store

## Constraints

- Prefix the file path in the asset tags in the md files with an env var read from a config file (`![]($$STATIC_ASSET_PATH/static_assets/image.jpg)`)
    - Can be added as a feature to anna, to support dynamic injection during render
    - The env var will be set to `.` during dev mode, to serve files locally
    - The var will be set to the obj store URL during prod render

The title of this project was inspired by [Nestor](https://tintin.fandom.com/wiki/Nestor), a fictional butler from the Tintin series.
