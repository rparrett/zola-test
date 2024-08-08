+++
title = "Typewriter Text Effect"
date = 2022-05-03
draft = false

[taxonomies]
categories = ["programming"]
tags = ["bevy"]

[extra]
lang = "en"
toc = true
comment = true
copy = true
display_tags = true
truncate_summary = false
+++

A common effect in games is to have text progressively appear letter-by-letter as if it is being actively typed. If all you need is a single line of text, you can achieve this with a simple system that adds letters to a `TextSection`. But this naive approach has issues when the text occupies multiple lines. When the "typewriter" runs out of room on a line of text while typing a word, that entire word will abruptly move to the next line!

We can get around this by laying out the entire contents of the text immediately but hiding it and progressively making a portion of it visible.

{% codeblock(name="Bevy 0.14") %}
{{ snippet(file="typewriter.rs") }}
{% end %}
