# Rhythmic shapes
## Demo
https://github.com/jamiegibney/rhythmic_shapes/assets/123845103/8dbb6e0e-1e11-4fae-ae1b-345650ef0bac

## Usage
The whole shape is treated as one bar.

- Click and drag nodes to any position.
- The "time signature" control changes the number of vertices, which acts as the number of beats per bar. This control allows between 3 and 8 vertices (inclusive).
- The "tempo" control changes the speed of the playhead in beats per minute (BPM).
- Press "R" to reset the shape and playhead position.
- Alt-click the tempo/time signature controls to reset them to their default values (120 BPM and 4/4, respectively).

## Concept

In essence: the playhead position is incremented linearly and continuously, and the distance between points is used to track when the playhead "taps" a node.

> Please note that, as per [#1](https://github.com/jamiegibney/rhythmic_shapes/issues/1), the current system may skip nodes if they are "stepped over" in a frame.

---

#### Playhead
The playhead "progress" is continuously updated each cycle. This value should be a value between `0.0` and `1.0`, and should wrap around if it ever exceeds `1.0`. Ideally, the playhead progress is incremented based on the tempo, which can be done following this formula:

$$
b=\frac{60}{4\cdot\mathrm{tempo}}\\
$$

$$
t=\frac{1}{b}\cdot T
$$

$$
p=(p+t)\mod1
$$

Here, $b$ represents the time per bar in seconds. $T$ is the time interval between calls in seconds. $t$ is the amount to increment the playhead progress. $p$ is the playhead progress.

$\mod1$ is the "modulo 1" function, essentially meaning the decimal part of a number. ($1.5\mod1=0.5$)

> *Note*: in this device, the time between calls ($T$) is the time between each frame, but this may also be the time between samples, in which case you would use $T=$ `1.0 / sample_rate`.

---

#### Nodes
Nodes — the two-dimensional points — may be placed wherever you choose. To distribute them uniformly to create regular shapes, see the below pseudocode:
```rust
// Our array of nodes, which are two-dimensional points:
let nodes = [ ... ]

// Some radius:
let radius = 250

// Divide 2π (tau) by the number of nodes:
let delta = 2π / number_of_nodes

// We iterate through all our nodes with a variable "i":
for i in 0 to number_of_nodes - 1:
    // We count backwards so that we go clockwise:
    let index = number_of_nodes - i

    // Multiply the index and the delta together, and then add π / 2
    // to rotate the shape 90º anti-clockwise so we start from the
    // top, not the right:
    let delta_angle = index * delta + (π / 2)

    // cos() gets us our x (horizontal) position...
    let x = cos(delta_angle) * radius

    // and sin() gets us our y (vertical) position
    let y = sin(delta_angle) * radius

    // Then, set the i'th node to (x, y):
    node[i] = (x, y)
end
```

See [`emplace_nodes()`](./src/ui/shape/mod.rs#L219) for the actual implementation in this device.

---

Line segments are created between consecutive nodes, which can be used to track distance. Adding the length of all segments together will yield the total length (perimeter) of the current shape. This is needed to both maintain a consistent playhead speed and identify when a node has been passed, or "tapped".

Significantly, in this device each node holds "`NoteEventData`", which may store any information you wish. In this example it simply encodes a MIDI note value, but could also, for example, encode filter cutoff frequency, distortion drive, or even a reference to an audio file for playback. 

Whenever a node is "tapped", the sequencer requests its `NoteEventData`, which is then processed further (the timing of the event is attached), and then sent to the audio thread to do its thing.

---

#### Process
This device follows the following process for finding when a node has been "tapped":

- Calculate and store the length of all segments. This can be done with the hypotenuse: $\sqrt{(b.x-a.x)^2+(b.y-a.y)^2}$, where $a$ and $b$ are two 2-dimensional points.
- Calculate and store the total length of all segments (i.e. the perimeter).
- Multiply the playhead progress (a value between `0.0` and `1.0`) by the total length.
- With this value, find the nearest two nodes to the playhead (what lengths is it between?).
- Check if the node "behind" the playhead has changed: if it has, then the **new** "behind" node has just been tapped. This node should be stored for the next call, so you can check if it changes again.

---

#### Playhead position

To find the position of the playhead between nodes (if you want to visualise it, for instance):

1. Find the distance between nodes (let's call it $\mathrm{dist}$) via inverse linear interpolation (inverse lerp):

$$
\mathrm{dist}=\frac{l-l_\mathrm{behind}}{l_\mathrm{ahead}-l_\mathrm{behind}}
$$

where:
- $l$ is the *length* from the beginning of the shape to the playhead position,
- $l_\mathrm{behind}$ is the *length* from the beginning of the shape to the node "behind" the playhead,
- $l_\mathrm{ahead}$ is the *length* from the beginning of the shape to the node "ahead of" the playhead.

</br>

2. Interpolate between the two nodes via linear interpolation (lerp):

$$
\mathrm{position}=\mathrm{dist}\cdot(p_\mathrm{ahead}-p_\mathrm{behind})+p_\mathrm{behind}
$$

where:
- $\mathrm{dist}$ is the interpolation factor from the previous equation,
- $p_\mathrm{behind}$ is the *position* of the node "behind" the playhead,
- $p_\mathrm{ahead}$ is the *position* of the node "ahead of" the playhead.
