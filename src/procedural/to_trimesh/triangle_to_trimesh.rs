use geom::Triangle;
use procedural::{ToTriMesh, TriMesh};
use math::{Scalar, Vect};

impl ToTriMesh<()> for Triangle {
    fn to_trimesh(&self, _: ()) -> TriMesh<Scalar, Vect> {
        TriMesh::new(
            vec!(self.a().clone(), self.b().clone(), self.c().clone()),
            None,
            None,
            None)
    }
}