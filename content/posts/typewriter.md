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

A common effect in games is to have text progressively appear letter-by-letter as if it were being typed by a human. If all you need is a single line of text, you can achieve this with a simple system that adds letters to a `Text` entity. However, this naive approach has an issue when the text occupies multiple lines. When the "typewriter" runs out of room on a line of text while typing a word, the layout gets adjusted. This causes the entire word to abruptly move to the next line.

We *could* get around this by using `LineBreak::AnyCharacter`, but splitting words over multiple lines makes text awkward to read.

A better approach is to lay out the entire contents of the text immediately, but with `Color::NONE`. We can then progressively make each character visible.

{% codeblock(name="Bevy 0.15") %}
{{ snippet(file="bevy_0_15/examples/typewriter.rs", anchor="content") }}
{% end %}
