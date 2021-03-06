// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::cli_state::CliState;
use crate::StarcoinOpt;
use anyhow::Result;
use scmd::{CommandAction, ExecContext};
use starcoin_types::peer_info::PeerId;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Default)]
#[structopt(name = "start")]
pub struct StartOpt {
    #[structopt(short = "f", long = "force")]
    /// if force is set, will cancel current sync task.
    force: bool,

    /// if peers is not empty, will try sync with the special peers.
    #[structopt(short = "p", long = "peer")]
    peers: Option<Vec<PeerId>>,
}

pub struct StartCommand;

impl CommandAction for StartCommand {
    type State = CliState;
    type GlobalOpt = StarcoinOpt;
    type Opt = StartOpt;
    type ReturnItem = ();

    fn run(
        &self,
        ctx: &ExecContext<Self::State, Self::GlobalOpt, Self::Opt>,
    ) -> Result<Self::ReturnItem> {
        let client = ctx.state().client();
        client.sync_start(
            ctx.opt().force,
            ctx.opt().peers.as_ref().cloned().unwrap_or_default(),
        )
    }
}
