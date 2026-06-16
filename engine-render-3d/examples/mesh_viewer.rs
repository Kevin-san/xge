//! GLTF 网格查看器示例
//!
//! 命令行加载 GLTF/GLB 模型并展示其结构。
//!
//! 用法: `cargo run --example mesh_viewer --features="gltf-loader" -- <path>`

#[cfg(feature = "gltf-loader")]
fn main() {
    use engine_render_3d::GltfModel;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: mesh_viewer <path-to-gltf-or-glb>");
        println!("\n如果没有指定文件，将创建一个简单的测试场景：");
        demo_without_file();
        return;
    }

    let path = &args[1];
    println!("=== Mesh Viewer ===");
    println!("Loading: {}\n", path);

    match GltfModel::from_file(path) {
        Ok(model) => {
            print_model_info(&model);
        }
        Err(e) => {
            eprintln!("Error loading model: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "gltf-loader")]
fn print_model_info(model: &engine_render_3d::GltfModel) {
    println!("Model loaded successfully!\n");

    println!("Meshes: {}", model.meshes.len());
    for (i, mesh) in model.meshes.iter().enumerate() {
        println!("  Mesh {}: {} vertices, {} triangles, {} primitives",
            i, mesh.vertices(), mesh.triangles(), mesh.primitive_count());

        let aabb = mesh.aabb();
        println!("    AABB: min={:?}, max={:?}", aabb.min(), aabb.max());
        println!("    Has normals: {}, Has tangents: {}",
            mesh.has_normals(), mesh.has_tangents());
    }

    println!("\nMaterials: {}", model.materials.len());
    for (i, mat) in model.materials.iter().enumerate() {
        println!("  Material {}: '{}'", i, mat.name());
        println!("    Base color: {:?}", mat.base_color());
        println!("    Metallic: {}, Roughness: {}", mat.metallic(), mat.roughness());
    }

    println!("\nNodes: {}", model.nodes.len());
    for (i, node) in model.nodes.iter().enumerate() {
        println!("  Node {}: '{}'", i, node.name);
        println!("    Translation: {:?}", node.translation);
        println!("    Scale: {:?}", node.scale);
        if let Some(mesh_idx) = node.mesh {
            println!("    Mesh: {}", mesh_idx);
        }
        println!("    Children: {:?}", node.children);
    }

    println!("\nScene roots: {:?}", model.scene_roots);

    let aabb = &model.aabb;
    println!("\nGlobal AABB: min={:?}, max={:?}", aabb.min(), aabb.max());
    let size = aabb.size();
    println!("  Size: {}x{}x{}", size.x, size.y, size.z);
}

#[cfg(feature = "gltf-loader")]
fn demo_without_file() {
    use engine_math::Vec3;
    use engine_render_3d::Mesh3D;

    println!("\nDemo: Basic primitives");
    println!("{:-<50}", "");
    let primitives = vec![
        ("Cube", Mesh3D::cube(1.0)),
        ("Sphere", Mesh3D::sphere(1.0, 16, 8)),
        ("Cylinder", Mesh3D::cylinder(0.5, 1.0, 16)),
        ("Cone", Mesh3D::cone(0.5, 1.0, 16)),
        ("Torus", Mesh3D::torus(0.7, 0.3, 16, 8)),
    ];

    for (name, mesh) in primitives {
        let aabb = mesh.aabb();
        println!("  {}: {} verts, AABB size: {}x{}x{}",
            name, mesh.vertices(), aabb.size().x, aabb.size().y, aabb.size().z);
    }

    println!("\n提供 GLTF/GLB 文件路径来加载实际模型。");
}

#[cfg(not(feature = "gltf-loader"))]
fn main() {
    eprintln!("This example requires the 'gltf-loader' feature to be enabled.");
    eprintln!("Run with: cargo run --example mesh_viewer --features=\"gltf-loader\" -- <path>");
    std::process::exit(1);
}
