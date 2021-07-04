use log::info;
use unity_native_plugin::profiler::*;
use unity_native_plugin::profiler_callbacks::*;

unity_native_plugin::unity_native_plugin_entry_point! {
    fn unity_plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
        plugin_load(interfaces);
    }

    fn unity_plugin_unload() {
        info!("unity_plugin_unload");
    }
}

fn plugin_load(interfaces: &unity_native_plugin::interface::UnityInterfaces) {
    env_logger::try_init().ok();

    info!("unity_plugin_load");

    let profiler_cb = interfaces
        .interface::<UnityProfilerCallbacks>()
        .expect("UnityProfilerCallbacks");

    profiler_cb.register_create_category(Box::new(|desc| {
        info!("create_category: desc={:?}", desc);
    }));

    profiler_cb.register_create_marker(Box::new(move |desc| {
        info!("create_marker: desc={:?}", desc);

        let flags = desc.flags();
        if flags.has_flag(ProfilerMarkerFlag::ScriptEnterLeave)
            || flags.has_flag(ProfilerMarkerFlag::VerbosityInternal)
        {
            return;
        }

        profiler_cb.register_marker_event(
            desc,
            Box::new(|desc| {
                info!("marker_event: desc={:?}", desc);
            }),
        );
    }));
}
