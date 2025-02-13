+++
title = "Drawing Lines in 2D"
date = 2023-10-18
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

Bevy 0.11 added an [immediate mode gizmos API](https://docs.rs/bevy/latest/bevy/prelude/struct.Gizmos.html#method.line) that can draw lines.

The immediate mode API is useful for quick debugging, but it has some limitations:

- Gizmos are always drawn on top of other content
- Gizmos aren't retained between frames. You must write code that draws them every frame.

If you want to just "spawn a line segment entity," you can use a sprite! A line segment is just a very skinny rectangle, after all.

{% codeblock(name="Bevy 0.15") %}
{{ snippet(file="bevy_0_15/examples/drawing_lines_sprite.rs", anchor="content") }}
{% end %}

Another way to do this would be to create a custom [`Mesh`](https://docs.rs/bevy/latest/bevy/render/prelude/struct.Mesh.html). A really handy plugin that takes this approach is [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon). It does tesselation with [lyon](https://github.com/nical/lyon) and supports polylines with nice joinery and other arbitrary 2d shapes.

{% note(header='Note') %}
Creating a unique mesh for each line segment you want to draw will be less performant that using sprites, because Bevy won't be able to use automatic batching.
{% end %}

Here's what DIYing it for a simple line segment looks like:

{% codeblock(name="Bevy 0.15") %}
{{ snippet(file="bevy_0_15/examples/drawing_lines_mesh.rs", anchor="content") }}
{% end %}

If you need more power, check out these options from the Bevy's third party ecosystem:

- [bevy_vector_shapes](https://github.com/james-j-obrien/bevy_vector_shapes)
- [bevy_vello](https://github.com/linebender/bevy_vello) (warning: no webgl2 support)
