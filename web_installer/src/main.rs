fn main() {
    println!("Hello, world!");
}

// KMods to link to the final img. Get from github
struct NeutronModule {
    url: String,
}

// Contains the path, options, etc
struct OSImg;

impl OSImg {
    fn link(_mod: NeutronModule) {

    }

    fn make_img(neutron_modules: &[&NeutronModule]) {
        let res = OSImg;
        for n_mod in neutron_modules {
            res.link(n_mod)
        }
    }
}

// Use WEBUSB interface or somehow native interface
pub struct USBWriter;
