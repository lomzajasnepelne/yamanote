pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_one_field_from_oneof_is_set_at_a_time() {
        let mut ctl = protos::control::ControlMessage::new();
        let mut get_node_stats = protos::control::GetNodeStats::new();
        get_node_stats.set_node_stat_params(123);
        ctl.set_get_node_stats(get_node_stats);
        let mut get_mesh_info = protos::control::GetMeshInfo::new();
        get_mesh_info.set_mesh_info_params(321);
        assert_eq!(ctl.has_get_node_stats(), true);
        assert_eq!(ctl.has_get_mesh_info(), false);
        ctl.set_get_mesh_info(get_mesh_info);
        assert_eq!(ctl.has_get_node_stats(), false);
        assert_eq!(ctl.has_get_mesh_info(), true);
    }
}
