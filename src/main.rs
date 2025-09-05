use bevy::{
    core_pipeline::prepass::DepthPrepass,
    pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt},
    prelude::*,
    render::mesh::SphereMeshBuilder,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(3., 2., 3.)).looking_at(Vec3::ZERO, Vec3::Y),
        DepthPrepass,
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::new(0., 5., 5.)).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let mat: Handle<StandardMaterial> = asset_server.add(Color::WHITE.into());

    commands.spawn((
        Mesh3d(asset_server.add(Plane3d::new(Vec3::Y, Vec2::splat(5.)).into())),
        MeshMaterial3d(mat.clone()),
    ));

    let sphere = asset_server.add(SphereMeshBuilder::default().ico(4).unwrap());

    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(Vec3::new(-0.5, 0.5, 0.5)),
    ));
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(Vec3::new(0.5, 0.5, 0.5)),
    ));
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(Vec3::new(-0.5, 0.5, -0.5)),
    ));
    commands.spawn((
        Mesh3d(sphere.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_translation(Vec3::new(0.5, 0.5, -0.5)),
    ));

    let decal_mat: Handle<ForwardDecalMaterial<StandardMaterial>> =
        asset_server.add(ForwardDecalMaterial {
            base: Color::srgb(1., 0., 0.).into(),
            extension: ForwardDecalMaterialExt {
                depth_fade_factor: 1.0,
            },
        });

    commands.spawn((
        ForwardDecal,
        MeshMaterial3d(decal_mat),
        Transform::from_scale(Vec3::splat(4.)),
    ));
}
