# Buzzy Browser

The Buzzy Browser is an experimental web browser. It is currently in the early
research phase with not much to show.

## Vision, Ideas and Motivation

The vision is to build a fast web browser with the newest technology, hence the
name Buzzy Browser.

The main ideas are:

1. Use a game engine with a data-oriented design.
2. Design for the future.
3. Idiomatic pure Rust project.

The plan is to use the Bevy game engine as a foundation. Browsers are very
similar to computer games technology-wise. For example, they have a scene graph
(DOM), scripting (JavaScript), graphics, animation, video, UI, sound, input and
networking. Games engines are often designed with performance in mind which we
believe could be helpful.

Designing for the future makes sense when starting out a new long term project.
There are already several great browsers that work very well. The idea is to do
something different. The existing browsers were designed a long time ago and
have a lot of baggage. We believe that there is an opportunity for improvement
by letting go of the past.

We believe Rust is likely the future of systems programming, and that is the
best choice for writing browsers that needs to be safe and fast. Rust has a
large amount of open source crates (libraries) that can be built on top on.

## Non-goals

Some non-goals are:

1. Compatibility with non HTML5 pages.
2. Compatibility with non-conforming web pages.
3. Compatibility of with old graphics APIs.
4. Compatibility of with old machines.
5. Compatibility with old operating systems.
6. Hurrying to market.

It does not make sense to strive for compatibility with quirky old web pages
when designing for the future. The quirky old web pages will most likely not be
visited by the vast majority of users. There is a cost associated with
supporting old web pages or web pages that are full of errors and quirks. That
cost can affect the speed of the browser and the time to develop the browser.

We suggest the users keep an established browser around if they need to open an
old quirky web page.

## Why not Servo?

The Servo web engine has been very successful. Several parts have been
integrated into the Firefox browser, and the project has moved beyond Mozilla
to be part of the Linux Foundation.

Servo pioneered some new ideas. For example, game-like rendering, increased
multi-threading and task based processing. But it is traditional in some ways.
It was designed to be integrated into Firefox so it inherited several designs.
It shares a large portion of C++ code as well, for example SpiderMonkey
and Angle. Some of these parts could eventually be replaced though, and things
might change now that the project is no longer part of Mozilla.
