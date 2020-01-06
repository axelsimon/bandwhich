use ::std::collections::HashMap;

use crate::network::Connection;

use super::lsof_utils;
use std::net::SocketAddr;

#[derive(Debug)]
struct RawConnection {
    ip: String,
    local_port: String,
    remote_port: String,
    protocol: String,
    process_name: String,
}

pub(crate) fn get_open_sockets() -> HashMap<Connection, String> {
    let mut open_sockets = HashMap::new();

    let connections = lsof_utils::get_connections();

    for raw_connection in connections {
        let protocol = raw_connection.get_protocol();
        let remote_ip = raw_connection.get_remote_ip();
        let local_ip = raw_connection.get_local_ip();
        let remote_port = raw_connection.get_remote_port();
        let local_port = raw_connection.get_local_port();

        let socket_addr = SocketAddr::new(remote_ip, remote_port);
        let connection = Connection::new(socket_addr, local_ip, local_port, protocol).unwrap();

        open_sockets.insert(connection, raw_connection.process_name.clone());
    }

    return open_sockets;
}
