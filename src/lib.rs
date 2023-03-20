// prevent the compiler warnings because it'd prefer `kv` to be named `Kv` instead
#![allow(non_camel_case_types)]

varnish::boilerplate!();
use std::error::Error;
use std::fs::File;
use std::borrow::Cow;
use uaparser::Parser;
use uaparser::{Client, UserAgentParser};
use varnish::vcl::ctx::Ctx;
use varnish::vcl::vpriv::VPriv;

varnish::vtc!(test01);

pub struct parser {
    p: UserAgentParser,
}

fn get_str<'a>(s: &'a Option<Cow<'a, str>>) -> &str {
    &s.as_ref().map(|s| s.as_ref()).unwrap_or("")
}

// implementation needs the same methods as defined in the vcc, plus "new()"
// corresponding to the constructor, which requires the context (_ctx) , and the
// name of the object in VLC (_vcl_name)
impl parser {
    // constructor doesn't need a Ctx, or the VCL name, hence the _ prefix
    pub fn new(_ctx: &Ctx, _vcl_name: &str, path: &str) -> Result<Self, Box<dyn Error>> {
        Ok(parser {
            p: UserAgentParser::from_file(File::open(path)?)
                .map_err(|e| varnish::vcl::Error::from(e.to_string()))?,
        })
    }

    // to be more efficient and avoid duplicating the string result just to
    // pass it to the boilerplate code, we can do the conversion to a VCL_STRING ourselves
    pub fn get<'a>(&'a self, _ctx: &mut Ctx, priv_parser: &'a VPriv<Client>, key: &str) -> &'a str {
        match priv_parser.as_ref() {
            None => "",
            Some(client) => {
                match key {
                    "device.family" => &client.device.family,
                    "device.brand" => get_str(&client.device.brand),
                    "device.model" => get_str(&client.device.model),
                    "os.family" => &client.os.family,
                    "os.major" => get_str(&client.os.major),
                    "os.minor" => get_str(&client.os.minor),
                    "os.patch" => get_str(&client.os.patch),
                    "os.patch_minor" => get_str(&client.os.patch_minor),
                    "ua.family" => &client.user_agent.family,
                    "ua.major" =>  get_str(&client.user_agent.major),
                    "ua.minor" =>  get_str(&client.user_agent.minor),
                    _ => "",
                }
            }
        }
    }

    pub fn parse<'a>(&self, _ctx: &mut Ctx, priv_parser: &'a mut VPriv<Client<'a>>, ua: &'a str) {
        priv_parser.store(self.p.parse(ua))
    }
}
