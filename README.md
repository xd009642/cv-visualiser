# cv-visualiser

## Introduction

This is a repository for the planned computer vision visualiser. This is
currently a more nebulous idea in my mind and likely to be unstable for
a while.

Inspired by tokio-console this aims to provide introspection into computer
vision pipelines to aid in debugging the algorithms in a visual manner which
hasn't really been acheived before.

## Design

Similar to tokio-console I expect this to be done via tracing, tracing is
widely used for introspection and tracing applications. Additionally, when
spans and events are filtered out the expressions for the fields won't be
evaluated. Meaning we can place large images as field expressions and only pay
the transfer cost in the event we enable the visualiser.

This means we can start to put the matrices containing out image data or
features into tracing events and get an overview of every transformation
applied and the stage it occurs in. What's more, the spans can add a hierarchy
grouping the matrices by the algorithm they occur in.

I anticipate using protobuf to transfer the values and then a frontend app
displays them in a timeline. Then something like the jaeger dashboard for
viewing spans and when you select 1..N spans you can see all the values
received in sequence with the field names.

## Prior Art

* [Bonsai](https://open-ephys.org/bonsai)
* [Opentelemetry](https://opentelemetry.io/)
* [ImageWatch](https://docs.opencv.org/2.4/doc/tutorials/introduction/windows_visual_studio_image_watch/windows_visual_studio_image_watch.html#windows-visual-studio-image-watch)
* [Tracing](https://github.com/tokio-rs/tracing)
* [Valuable](https://github.com/tokio-rs/valuable)
* [tokio-console](https://github.com/tokio-rs/console)
* [robokite PoC](https://github.com/alsuren/robokite/pull/1)
* [Zooming a billion trace events at 60fps](https://thume.ca/2021/03/14/iforests)
