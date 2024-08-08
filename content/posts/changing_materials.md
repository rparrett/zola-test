+++
title = "Changing a Material"
date = 2023-10-06
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

When you load a scene from a glTF and spawn it multiple times, the material handles in each instance of the scene point to the same material asset.

If you want to modify the material properties of only one of these instances, you can clone it, modify it, and add it to the asset server.

{% codeblock(name="Bevy 0.14") %}
{{ snippet(file="changing_materials.rs", anchor="content") }}
{% end %}
