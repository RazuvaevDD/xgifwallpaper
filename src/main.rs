extern crate x11;

use gift::Decoder;
use gift::decode::Steps;

use std::ptr;

use std::ffi:: {
    CString,
    c_void
};

use std::fs::File;

use std::io::BufReader;

use std::os::raw:: {
    c_char
};

use x11::xlib::*;


// TODO Center-Feature
// TODO Scale-Feature
// TODO Fill-Feature
// TODO On all Screens
// TODO Multiple images on different screens
// TODO Minimize unsafe
// TODO cmd-opts
// TODO Verbose mode
// TODO Tiling?

fn load_gif(filename: String) -> Steps<BufReader<File>>
{
    let file_in = File::open(filename)
        .expect("Could not load gif");

    let decoder = Decoder::new(file_in);

    return decoder.into_steps();
}

fn loop_animation(steps: Steps<BufReader<File>>)
{
    unsafe {
        let xdisplay = XOpenDisplay(ptr::null());

        let (images, _rasters) = convert_frames_to_ximages(xdisplay, steps);

        // TODO Loop Frames with delay
        for ximage_ptr  in images.iter() {
            println!("ximage_ptr: {:?}", ximage_ptr);
            set_image_as_background(xdisplay, *ximage_ptr);
        }
        
        // TODO React on signal in loop

        // TODO Update background in loop via pixmap

        XCloseDisplay(xdisplay);
    }
}

fn convert_frames_to_ximages(
    xdisplay: *mut Display, mut frames: Steps<BufReader<File>>) 
    -> (Vec<XImage>, Vec<Vec<c_char>>)
{
    let mut frame_count = 0;

    // TODO Loop over frames
    let frame = frames.nth(0)
        .unwrap()
        .expect("Empty frame in animation");
    
    let raster = frame.raster();
    
    frame_count = frame_count + 1;
    println!("Frame {}", frame_count);
    println!("delay: {}", frame.delay_time_cs().unwrap_or(666));
    println!("width: {:?}", raster.width());
    println!("height: {:?}", raster.height());

    let mut image_structs: Vec<XImage> = Vec::new();
    let mut image_rasters: Vec<Vec<c_char>> = Vec::new();

    unsafe {
        let xscreen = XDefaultScreenOfDisplay(xdisplay);
        let xvisual = XDefaultVisualOfScreen(xscreen);

        let ximage = XCreateImage(
            xdisplay,
            xvisual,
            24,
            ZPixmap,
            0,
            ptr::null_mut(), 
            raster.width(),
            raster.height(),
            32,
            0 
        );

        let data_size = ((*ximage).bytes_per_line * (*ximage).height) as usize;
        let mut data: Vec<c_char> = Vec::with_capacity(data_size);

        for pixel_channel in raster.as_u8_slice() {
            data.push(*pixel_channel as i8);
        }

        println!("data.len: {} == data_size: {}", data.len(), data_size);

        let data_ptr = data.as_mut_ptr();
        (*ximage).data = data_ptr;       
   
        image_structs.push(*ximage);
        image_rasters.push(data);
    }

    return (image_structs, image_rasters);
}

fn set_image_as_background(display: *mut Display, ximage: XImage) -> XImage {
    unsafe {
        let screen = XDefaultScreen(display);
        let gc = XDefaultGC(display, 0);
        let width = XDisplayWidth(display, screen) as u32;
        let height = XDisplayHeight(display, screen) as u32;
        let root = XRootWindow(display, screen);
        let depth = XDefaultDepth(display, screen) as u32;

        XSetCloseDownMode(display, RetainPermanent);

        println!("{:?} {:?} {:?} {:?} {:?}", display, root, width, height, depth);
        println!("ximage: {:?}", *(ximage.data));

        let pixmap = XCreatePixmap(display, root, width, height, depth);
        XSync(display, False);

        let mut image = ximage;

        XPutImage(
            display,
            pixmap,
            gc,
            &mut image,
            0, 0, 0, 0,
            ximage.width as u32, ximage.height as u32
        );

        XFlush(display);

        if !set_root_atoms(display, root, pixmap) {
            println!("set_root_atoms failed!");
        }

        XSetWindowBackgroundPixmap(display, root, pixmap);
        XClearWindow(display, root);
        XFlush(display);
        
        return image;
    }
}

fn main() {
    // TODO Read Args
    let gif_filename = String::from("/home/frank/Pictures/sample.gif");
    
    // TODO Analyze screen-count and resolutions
   
    // Load GIF
    let steps = load_gif(gif_filename);

    // TODO Scale GIF-Frames accordingly to params (Center, Scale, Fill)
    loop_animation(steps);
}

unsafe fn demo() 
{
    // Open display connection.
    let display = XOpenDisplay(ptr::null());

    if display.is_null() {
        panic!("XOpenDisplay failed");
    }

    println!("ScreenCount: {}", XScreenCount(display));

    let screen = XDefaultScreen(display);
    let width = XDisplayWidth(display, screen) as u32;
    let height = XDisplayHeight(display, screen) as u32;
    let root = XRootWindow(display, screen);
    let cmap = XDefaultColormap(display, screen);
    let depth = XDefaultDepth(display, screen) as u32;

    let mut color = XColor {
        pixel: 0,
        red: 32000,
        green: 64000,
        blue: 32000,
        flags: DoRed | DoGreen | DoBlue,
        pad: 0
    };

    let color_ptr: *mut XColor = &mut color;

    println!("display: {:?}", display);

    XAllocColor(display, cmap, color_ptr);

    println!("display: {:?}", display);
    
    let pixmap = XCreatePixmap(display, root, width, height, depth);

    println!("display: {:?}", display);

    let mut gcvalues = XGCValues {
        function: GXcopy,
        plane_mask: XAllPlanes(),
        foreground: color.pixel,
        background: color.pixel,
        line_width: 0,
        line_style: 0,
        cap_style: 0,
        join_style: 0,
        fill_style: 0,
        fill_rule: 0,
        arc_mode: 0,
        tile: 0,
        stipple: 0,
        ts_x_origin: 0,
        ts_y_origin: 0,
        font: 0,
        subwindow_mode: ClipByChildren,
        graphics_exposures: True,
        clip_x_origin: 0,
        clip_y_origin: 0,
        clip_mask: 0,
        dash_offset: 0,
        dashes: 0,
    };
    
    let gc_ptr: *mut XGCValues = &mut gcvalues;
    let gc_flags = (GCForeground | GCBackground) as u64;
    let gc = XCreateGC(display, root, gc_flags, gc_ptr);

    XFillRectangle(display, pixmap, gc, 0, 0, width, height);
    XFreeGC(display, gc);
    
    println!("display: {:?}", display);
    println!("screen: {}", screen);
    println!("depth: {}", depth);
    println!("width: {}", width);
    println!("height: {}", height);
    println!("color.pixel: {}", color.pixel);

    if !set_root_atoms(display, root, pixmap) {
        println!("set_root_atoms failed!");
    }

    XSetWindowBackgroundPixmap(display, root, pixmap);
    XClearWindow(display, root);
    XFlush(display);
    XSetCloseDownMode(display, RetainPermanent);
    XCloseDisplay(display);
}

unsafe fn set_root_atoms(display: *mut Display, root: u64, pixmap: Pixmap) -> bool {
    let xrootmap_id = CString::new("_XROOTPMAP_ID").expect("Failed!"); 
    let esetroot_pmap_id = CString::new("ESETROOT_PMAP_ID").expect("Failed!"); 

    let mut atom_root = XInternAtom(display, xrootmap_id.as_ptr(), True);
    let mut atom_eroot = XInternAtom(display, esetroot_pmap_id.as_ptr(), True);

    // Doing this to clean up after old background.
    //
    // XInternAtom may return "None", but nowhere defined in bindigs? So I
    // use 0 as direct, known value of None. See X.h.
    if atom_root != 0 && atom_eroot != 0 {
        // TODO Better way to have an initialized, non-null pointer?
        let data_root = CString::new("").expect("Failed!"); 
        let mut data_root_ptr : *mut u8 = data_root.as_ptr() as *mut u8;

        let data_eroot = CString::new("").expect("Failed!");
        let mut data_eroot_ptr : *mut u8 = data_eroot.as_ptr() as *mut u8;

        let mut ptype = 0 as u64;
        let mut format = 0 as i32;
        let mut length = 0 as u64;
        let mut after = 0 as u64;

        let result = XGetWindowProperty(display, root, atom_root, 0, 1, False, AnyPropertyType as u64, &mut ptype, &mut format, &mut length, &mut after, &mut data_root_ptr);

        if result == Success as i32 && ptype == XA_PIXMAP {
            XGetWindowProperty(display, root, atom_eroot, 0, 1, 0, AnyPropertyType as u64, &mut ptype, &mut format, &mut length, &mut after, &mut data_eroot_ptr);

            let root_pixmap_id = *(data_root_ptr as *const Pixmap);
            let eroot_pixmap_id = *(data_eroot_ptr as *const Pixmap);

            // Why the data_root-conversion to pixmap for equality-check???
            if // *data_root > 0 
               //  && *data_eroot > 0 
                 ptype == XA_PIXMAP 
                && root_pixmap_id == eroot_pixmap_id {

                XKillClient(display, root_pixmap_id);
                XFree(data_eroot_ptr as *mut c_void);
            }

            XFree(data_root_ptr as *mut c_void);
        }
    }

    atom_root = XInternAtom(display, xrootmap_id.as_ptr(), 0);
    atom_eroot = XInternAtom(display, esetroot_pmap_id.as_ptr(), 0);

    if atom_root == 0 || atom_eroot == 0 {
        return false;
    }

    // setting new background atoms
    let pixmap_ptr: *const Pixmap = &pixmap;
    
    XChangeProperty(display, root, atom_root, XA_PIXMAP, 32, PropModeReplace, pixmap_ptr as *const u8, 1);
    XChangeProperty(display, root, atom_eroot, XA_PIXMAP, 32, PropModeReplace, pixmap_ptr as *const u8, 1);

    return true;
}
