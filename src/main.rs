mod nifti;

fn main() {
    let image = nifti::nifti_io::read_nifti_image("test-cases/test.nii", 1);
    if let Some(image) = image {
        println!("Successfully read NIFTI image");
        println!("The dimensions are {}, {}, {}", image.nx, image.ny, image.nz);
        println!("There are a total of {} voxels", image.nvox)
    } else {
        println!("Failed to read NIFTI image.");
    }
}