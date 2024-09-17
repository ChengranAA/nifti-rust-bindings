// AI generated struct conversion from C to Rust 

#[repr(C)]
pub struct NiftiImage {
    pub ndim: i64,              // int64_t ndim;
    pub nx: i64,                // int64_t nx;
    pub ny: i64,                // int64_t ny;
    pub nz: i64,                // int64_t nz;
    pub nt: i64,                // int64_t nt;
    pub nu: i64,                // int64_t nu;
    pub nv: i64,                // int64_t nv;
    pub nw: i64,                // int64_t nw;
    pub dim: [i64; 8],          // int64_t dim[8];
    pub nvox: i64,              // int64_t nvox;
    pub nbyper: i32,            // int nbyper;
    pub datatype: i32,          // int datatype;

    pub dx: f64,                // double dx;
    pub dy: f64,                // double dy;
    pub dz: f64,                // double dz;
    pub dt: f64,                // double dt;
    pub du: f64,                // double du;
    pub dv: f64,                // double dv;
    pub dw: f64,                // double dw;
    pub pixdim: [f64; 8],       // double pixdim[8];

    pub scl_slope: f64,         // double scl_slope;
    pub scl_inter: f64,         // double scl_inter;

    pub cal_min: f64,           // double cal_min;
    pub cal_max: f64,           // double cal_max;

    pub qform_code: i32,        // int qform_code;
    pub sform_code: i32,        // int sform_code;

    pub freq_dim: i32,          // int freq_dim;
    pub phase_dim: i32,         // int phase_dim;
    pub slice_dim: i32,         // int slice_dim;

    pub slice_code: i32,        // int slice_code;
    pub slice_start: i64,       // int64_t slice_start;
    pub slice_end: i64,         // int64_t slice_end;
    pub slice_duration: f64,    // double slice_duration;

    pub quatern_b: f64,         // double quatern_b;
    pub quatern_c: f64,         // double quatern_c;
    pub quatern_d: f64,         // double quatern_d;
    pub qoffset_x: f64,         // double qoffset_x;
    pub qoffset_y: f64,         // double qoffset_y;
    pub qoffset_z: f64,         // double qoffset_z;
    pub qfac: f64,              // double qfac;

    pub qto_xyz: NiftiDmat44,   // nifti_dmat44 qto_xyz;
    pub qto_ijk: NiftiDmat44,   // nifti_dmat44 qto_ijk;

    pub sto_xyz: NiftiDmat44,   // nifti_dmat44 sto_xyz;
    pub sto_ijk: NiftiDmat44,   // nifti_dmat44 sto_ijk;

    pub toffset: f64,           // double toffset;

    pub xyz_units: i32,         // int xyz_units;
    pub time_units: i32,        // int time_units;

    pub nifti_type: i32,        // int nifti_type;

    pub intent_code: i32,       // int intent_code;
    pub intent_p1: f64,         // double intent_p1;
    pub intent_p2: f64,         // double intent_p2;
    pub intent_p3: f64,         // double intent_p3;
    pub intent_name: [u8; 16],  // char intent_name[16];

    pub descrip: [u8; 80],      // char descrip[80];
    pub aux_file: [u8; 24],     // char aux_file[24];

    pub fname: *mut u8,         // char *fname;
    pub iname: *mut u8,         // char *iname;
    pub iname_offset: i64,      // int64_t iname_offset;
    pub swapsize: i32,          // int swapsize;
    pub byteorder: i32,         // int byteorder;
    pub data: *mut std::ffi::c_void,  // void *data;

    pub num_ext: i32,           // int num_ext;
    pub ext_list: *mut Nifti1Extension,  // nifti1_extension *ext_list;
    pub analyze75_orient: Analyze75OrientCode, // analyze_75_orient_code analyze75_orient;
}

#[allow(dead_code)]
#[repr(C)]
pub struct NiftiDmat44 {
    pub m: [[f64; 4]; 4], // 4x4 matrix of doubles
}

#[allow(dead_code)]
#[repr(C)]
pub struct Nifti1Extension {
    pub esize: i32,             // int esize;
    pub ecode: i32,             // int ecode;
    pub edata: *mut u8,         // char *edata; (pointer to raw data)
}

#[allow(dead_code)]
#[repr(C)]  // Ensure C-compatible layout with C enums
pub enum Analyze75OrientCode {
    TransverseUnflipped = 0,    // a75_transverse_unflipped
    CoronalUnflipped = 1,       // a75_coronal_unflipped
    SagittalUnflipped = 2,      // a75_sagittal_unflipped
    TransverseFlipped = 3,      // a75_transverse_flipped
    CoronalFlipped = 4,         // a75_coronal_flipped
    SagittalFlipped = 5,        // a75_sagittal_flipped
    OrientUnknown = 6,          // a75_orient_unknown
}