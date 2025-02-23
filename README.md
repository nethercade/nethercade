# Nethercade

Primary working repository for Nethercade Z.

### Vertex Data:
Vertices must be passed to the console in the following format:
> [Pos.x, Pos.y, Pos.Z, Red, Green, Blue, Uv.U, Uv.V, Normal.X, Normal.Y, Normal.Z]

Pipelines which don't use those fields can be omitted. For example:

#### Color:
Needs Position, and Colors.

> [Pos.x, Pos.y, Pos.Z, Red, Green, Blue]

#### MatcapUv:
Needs Position, UVs, and Normals.

> [Pos.x, Pos.y, Pos.Z, Uv.U, Uv.V, Normal.X, Normal.Y, Normal.Z]

### Magic Numbers:
Pipelines:
```
0 => Color
1 => Uv
2 => ColorUv
3 => Quad2d
4 => Matcap
5 => MatcapColor
6 => MatcapUv
7 => MatcapColorUv
```

Blend Modes:
```
0 => No Blend
1 => Overwrite
2 => Add
3 => Screen
4 => Color Dodge
5 => Subtract
6 => Multiply
7 => Color Burn
8 => Overlay
```