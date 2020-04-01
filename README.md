# Mdblog

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![MIT/Apache2 licensed][license-image]
[![dependency status][deps-image]][deps-link]
[![Build Status][travis-image]][travis-link]

[crate-image]: https://img.shields.io/crates/v/morpho.svg
[crate-link]: https://crates.io/crates/morpho
[docs-image]: https://docs.rs/morpho/badge.svg
[docs-link]: https://docs.rs/morpho
[license-image]: https://img.shields.io/crates/l/morpho.svg
[deps-image]: https://deps.rs/repo/github/fugangqiang/morpho.rs/status.svg
[deps-link]: https://deps.rs/repo/github/fugangqiang/morpho.rs
[travis-image]: https://travis-ci.org/FuGangqiang/morpho.rs.svg?branch=master
[travis-link]: https://travis-ci.org/FuGangqiang/morpho.rs

Static site generator from markdown files with features:

* TeX style math support
* file path is the post url
* file name is the post title
* post can be hidden(link does not be insert into index/tag page)

you can check the [demo site](https://fugangqiang.github.io/morpho.rs/)
to learn the usages of morpho.


# Install

`morpho` is implemented by rust language(2018 edition), so you need cargo command:

```
cargo install morpho
```

`morpho` will be installed in your cargo binary directory(`~/.cargo/bin/`).


# CLI

`morpho` can be use as a command:

```
$ morpho -h
static site generator from markdown files

USAGE:
    morpho <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build    Build the blog static files
    help     Prints this message or the help of the given subcommand(s)
    init     Initialize the blog directory layout
    new      Create a blog post
    serve    Serve the blog, rebuild on change
    theme    Blog theme operations
```

you can also check the subcommand usage:

```
$ morpho serve -h
Serve the blog, rebuild on change

USAGE:
    morpho serve [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>    Serve the blog at http://127.0.0.1:<port> [default: 5000]
```


### init blog

```
$ morpho init myblog
```

blog directory(`myblog`) layout is initialized:

```
myblog
├── config.toml
├── media
├── posts
│   ├── hello.md
│   └── math.md
└── _themes
```

* `config.toml`: blog config file
* `media`: blog media directory
* `posts`: blog posts directory
* `posts/hello.md`: a markdown style post
* `_themes`: blog themes directory

### build blog

```
$ cd myblog
$ morpho build
```

the blog static files are build into the subdir `_build`, the current blog directory(`myblog`) layout is:

```
myblog
├── config.toml
├── media
├── posts
│   ├── hello.md
│   └── math.md
├── _themes
└── _builds
```

* `_builds`: generated static-site top directory

### serve blog

```
$ morpho serve
```

open the site index page automatically,
and re-generate your static-site when you add or change content,

### new post

create a new post titled `another`:

```
$ morpho new another
```

a new markdown file `posts/another.md` is created,
you can edit it for the new post.

refresh the index page, you will find the new post.


# config.toml

```toml
site_url = ""
site_name = "Mdblog"
site_motto = "Simple is Beautiful!"
footer_note = "Keep It Simple, Stupid!"
media_dir = "media"
build_dir = "_build"
theme = "simple"
theme_root_dir = "_themes"
rebuild_interval = 2
posts_per_page = 20
```
