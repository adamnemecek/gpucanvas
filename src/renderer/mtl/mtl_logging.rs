struct Labels {}
impl Labels {
    pub fn new() -> Self {
        todo!()
    }
}

pub struct MtlLog {
    inner: oslog::OSPoiLog,
    spid: oslog::OSSignpostID,
}

impl MtlLog {
    pub fn new() -> Self {
        todo!()
        // Self {
        //     inner: oslog::OSPoiLog::new("here"),
        //     spid:
        // }
    }
}

// let log = OSPoiLog::new("com.ngrid.app");
// let spid = log.new_spid();
// let cstr = CString::new("poilog").unwrap();
// // let s = "emit";
// let s = CString::new("emit").unwrap();
// log.start(spid, &cstr);
// for _ in 0..20 {
//     loop {
//         log.emit(spid, &s);
//         std::thread::sleep(std::time::Duration::from_millis(100));
//     }
// }

// log.end(spid, &cstr);
