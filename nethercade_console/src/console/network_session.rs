use std::net::SocketAddr;

use ggrs::{Config, P2PSession, PlayerType, SessionBuilder, UdpNonBlockingSocket};
use nethercade_core::Rom;
use serde::{Deserialize, Serialize};

use super::network::{NetworkInputState, WasmConsoleState};

#[derive(Serialize, Deserialize)]
pub struct NetworkSession {
    pub players: Box<[NetworkedClient]>,
}

#[derive(Serialize, Deserialize)]
pub struct NetworkedClient {
    pub count: usize,
    pub kind: NetworkedPlayerType,
}

#[derive(Serialize, Deserialize)]
pub enum NetworkedPlayerType {
    Local,
    Remote(SocketAddr),
}

#[derive(Debug)]
pub struct GgrsInstance;

impl Config for GgrsInstance {
    type Input = NetworkInputState;
    type Address = SocketAddr;
    type State = WasmConsoleState;
}

pub fn init_session(
    rom: &Rom,
    port: u16,
    players: &[PlayerType<SocketAddr>],
) -> P2PSession<GgrsInstance> {
    let mut sess_builder = SessionBuilder::new()
        .with_input_delay(rom.frame_rate.default_input_delay())
        .with_sparse_saving_mode(false)
        .with_num_players(players.len())
        .with_fps(rom.frame_rate.frames_per_second())
        .unwrap();

    for (id, address) in players.iter().enumerate() {
        sess_builder = sess_builder.add_player(*address, id).unwrap();
    }

    let socket = UdpNonBlockingSocket::bind_to_port(port).unwrap();
    sess_builder.start_p2p_session(socket).unwrap()
}
