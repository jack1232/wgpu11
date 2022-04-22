mod common;
mod vertex_data;

fn create_vertices(r: f32, u:usize, v: usize) -> Vec<common::Vertex> {
    let(pos, normal, _uvs) = vertex_data::sphere_data(r, u, v);
    let mut data:Vec<common::Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(common::vertex(pos[i], normal[i]));
    }
    data.to_vec()
}

fn main(){
    let vertex_data = create_vertices(2.0, 15, 20);
    let light_data = common::light([1.0,0.0,0.0], [1.0, 1.0, 1.0], 0.1, 0.6, 0.3, 30.0);
    common::run(&vertex_data, light_data, "sphere");
}
