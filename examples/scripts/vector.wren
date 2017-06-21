foreign class Vec3 {
  construct new(x, y, z) {}

  foreign toString

  foreign norm()
  foreign dot(rhs)
  foreign cross(rhs) // returns the result as a new vector

  // accessors
  foreign x
  foreign x=(x)
  foreign y
  foreign y=(y)
  foreign z
  foreign z=(z)
}