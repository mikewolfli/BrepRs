use breprs::visualization::virtual_texture;

fn main() {
    println!("Testing virtual_texture module...");
    let system = virtual_texture::VirtualTextureSystem::new();
    println!("VirtualTextureSystem created successfully!");
}
