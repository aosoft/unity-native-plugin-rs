use log::*;
use std::io::*;
use std::num::NonZeroU64;
use std::time::*;
use unity_native_plugin::profiler::*;
use unity_native_plugin::profiler_callbacks::*;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
        env_logger::try_init().ok();

        debug!("unity_plugin_load");
        plugin_load(interfaces);
    }

    fn unity_plugin_unload() {
        debug!("unity_plugin_unload");
    }
}

#[allow(unused)]
fn plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    let profiler_cb = interfaces
        .interface::<UnityProfilerCallbacks>()
        .expect("UnityProfilerCallbacks");

    profiler_cb.register_create_category(Box::new(|desc| {
        debug!("create_category: desc={:?}", desc);
    }));

    let (sender, receiver) = flume::unbounded::<String>();

    let start_ts = Instant::now();

    std::thread::spawn(move || {
        let file = std::fs::File::create("profile.log").unwrap();
        let mut writer = std::io::BufWriter::new(file);

        writer.write_all(b"[{}").ok();
        let mut count = 0;
        for msg in receiver.iter() {
            writer.write_all(msg.as_bytes()).ok();
            count += 1;

            let dt = Instant::now() - start_ts;
            if dt.as_secs() > 20 {
                info!("stop tracing, duration={:?}, events={}", dt, count);
                break;
            }
        }
        writer.write_all(b"]").ok();
    });

    profiler_cb.register_create_marker(Box::new(move |desc| {
        let tid = std::thread::current().id();
        debug!("create_marker: tid={:?}, desc={:?}", tid, desc);

        let flags = desc.flags();
        if flags.has_flag(ProfilerMarkerFlag::ScriptEnterLeave)
            || flags.has_flag(ProfilerMarkerFlag::VerbosityInternal)
        {
            return;
        }

        let sender = sender.clone();
        profiler_cb.register_marker_event(
            desc,
            Box::new(move |desc| {
                let ph = match desc.event_type {
                    ProfilerMarkerEventType::Begin => "B",
                    ProfilerMarkerEventType::End => "E",
                    ProfilerMarkerEventType::Single => "i",
                    _ => {
                        return;
                    }
                };
                let pid = 1;
                let tid: NonZeroU64 = unsafe { std::mem::transmute(std::thread::current().id()) };
                let ts = Instant::now();
                let dt = ts - start_ts;
                let cat = desc.desc.category_id();
                let cat: BuiltinProfilerCategory =
                    if cat <= BuiltinProfilerCategory::VirtualTexturing as u16 {
                        unsafe { std::mem::transmute(cat) }
                    } else {
                        BuiltinProfilerCategory::Other
                    };

                sender
                    .send(format!(
                        r#",{{"pid":1,"tid":{},"ph":"{}","ts":{},"cat":"{:?}","name":{:?}}}
"#,
                        tid,
                        ph,
                        dt.as_micros(),
                        cat,
                        desc.desc.name()
                    ))
                    .ok();

                for data in desc.event_data.iter() {
                    sender.send(format!("data={:?}", data.value())).ok();
                }
            }),
        );
    }));
}
