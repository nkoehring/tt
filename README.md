tt – track time
===============

Minimalist, manual, *offline* time tracker. Everything local, no cloud or any foreign computer involved.

If you want a fully automatic time tracker, you ended up in the wrong corner of the internet, sorry. Maybe you want to give [WakaTime](https://wakatime.com/) a try.

If you rather want to keep your data for yourself, I cannot offer you a automatic solution like WakaTime but a handy tool to track your time manually.

For my freelancing activities, I wanted an easy way to track my time without too much ~~convenience~~ surveillance involved. I used to create simple CSV files to track my hours per project and with `tt` I wanted to make this process less cumbersome and more colorful.

screenshot
----------

![screenshot](scrot.jpg)

Usage
-----

I assume that you somehow made the program available as `tt` in your PATH. See the [build section](#build)

`tt` can be used with simple local files or a more elaborate project configuration. Lets start with a local file for the basics, although I'm sure you will want to have a project configuration eventually.

Go into the folder where you want that report file and run `tt`. It will show you output similar to this:

```
No (existing/default) projects configured.
Trying to load local ./report.csv.
Couldn't load any report. Is your project configuration correct?
Consider adding a project to your config file (usually ~/.config/tt/config.toml),
or creating a file right here with `touch ./report.csv`
```

Lets listen to the computers wisdom and create an empty report file with `touch report.csv`. Now the output changes:

```
No (existing/default) projects configured.
Trying to load local ./report.csv.
Total 0.0 hours.
```

We didn't work at all yet. Again the computers wisdom tells us the obvious. So lets add some work we've done today:

```
% tt add 2.5 reading way too long README of tt
No (existing/default) projects configured.
Trying to load local ./report.csv.
1 entries added
./report.csv updated
```

Wow, you are a ~~slow~~ thorough reader:

```
% tt
No (existing/default) projects configured.
Trying to load local ./report.csv.
2017-10-31:  2.5h reading way too long README of tt
Total 2.5 hours.
```

`tt` gives btw the same output as `tt report`. Just to have mentioned it.

Nice. You added your first entry. As you can see, `tt` always uses the current date. There is also no way to change that (yet?).

Now if you want to add more than one entry at once, you can do that too:

```
% tt add
No (existing/default) projects configured.
Trying to load local ./report.csv.
Add entries line by line like ‘2.5 foo bar baz’ and press CTRL+d when finished
```

Now you can add entries line by line the same way you would do it with `tt add`:
```
0.5 reading way too long README of tt
0.5 this README is too long
ctrl+d
```

Now it will tell you that two more entries where added. If you run `tt` you'll see that all the entries are combined to one. That is because they are all on one day. `tt` also tries to combine the comments in a meaningful way by skipping double entries:

```
% tt
No (existing/default) projects configured.
Trying to load local ./report.csv.
2017-10-31:  3.5h reading way too long README of tt, this README is too long
Total 3.5 hours.
```

If you want to see all entries, use `tt all`:

```
% tt all
No (existing/default) projects configured.
Trying to load local ./report.csv.
2017-10-31:  2.5h reading way too long README of tt
2017-10-31:  0.5h reading way too long README of tt
2017-10-31:  0.5h this README is too long
Total 3.5 hours.
```

project configuration
---------------------

You probably got annoyed already by these first couple of lines talking about a missing default project and the local file. You can avoid that and get some more nice features of `tt` by adding a configuration. This usually sits in `~/.config/tt/config.toml` and looks like this:

```
default_project = "test"
default_rate = 50.0

[test]
path = "/home/fancypants/projects/test/report.csv"
rate = 55.0

[test2]
path = "/path/to/another/report.csv"
```

As you can see, there are projects and their payment rates. What all this means will show the next `tt` call, which you can do from whereever you want, if the path in your config is correct:

```
% tt
Loaded report from /home/fancypants/projects/test/report.csv
2017-10-31:  3.5h reading way too long README of tt, this README is too long

Project test: 3.5 hours total.
Total earnings are €192.50
```

Yes! You earned nearly 200€ already! Okay, of course that is very simplistic but for the typical hourly rated job it is enough. If you don't want to see a € symbol there, you can override that with the `currency` and `default_currency` settings.

The `default_project` setting here is a bit meaningless because without it, `tt` would just pick the first one in the list. With lots of projects this should be handy though. Just set it to the current project you are mostly working on.

If you need to reach another project, you can just prepend any command with the project name. So instead of `tt report` you call `tt test2 report` or just `tt test2`. Same with `tt test2 all` and `tt test2 add`.

For the people who scan this text for code blocks:

```sh
% tt project2 || tt project2 report # to print the report of "project2"
```

planned features
----------------

Currently editing and deleting of entries has to be done manually. I thought about an _edit mode_, that makes this more handy.

`tt` (`tt report`) summarizes single entries per day. I want to condense it further so that entries from last month are summarized for the whole month. Maybe even so much that only the last 5 days or so are shown (but usually the whole current month is of interest).

build
-----

There are no packages, yet. To "install", you need [Rust](https://www.rustup.rs/). I develop `tt` on nightly but it might just work fine with stable.

If you've installed Rust and Cargo (installed with Rustup), you can build tt yourself:

```sh
git clone https://github.com/nkoehring/tt.git
cd tt
cargo build --release
```

You'll find the program in `./target/release/`. You can "install" it by putting it into your $PATH. If you have a local bin folder, then you'd do for example:

```sh
cp target/release/tt ~/.local/bin/tt
```

Or use a link:

```sh
cd ~/.local/bin/
ln -s /path/to/tt/target/release/tt tt
```
