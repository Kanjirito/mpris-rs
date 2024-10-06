use std::collections::HashMap;

use zbus::{names::BusName, Connection};

use crate::{
    metadata::{MetadataValue, TrackID},
    proxies::{MediaPlayer2Proxy, PlayerProxy},
    LoopStatus, Metadata, Mpris, MprisError, PlaybackStatus,
};

pub struct Player {
    mp2_proxy: MediaPlayer2Proxy<'static>,
    player_proxy: PlayerProxy<'static>,
}

impl Player {
    pub async fn new<B>(
        mpris: &Mpris,
        bus_name: BusName<'static>,
    ) -> Result<Player, MprisError> {
        Player::new_from_connection(mpris.connection.clone(), bus_name).await
    }

    pub(crate) async fn new_from_connection(
        connection: Connection,
        bus_name: BusName<'static>,
    ) -> Result<Player, MprisError> {
        let mp2_proxy = MediaPlayer2Proxy::builder(&connection)
            .destination(bus_name.clone())?
            .build()
            .await?;

        let player_proxy = PlayerProxy::builder(&connection)
            .destination(bus_name.clone())?
            .build()
            .await?;

        Ok(Player {
            mp2_proxy,
            player_proxy,
        })
    }

    pub async fn metadata(&self) -> Result<Metadata, MprisError> {
        Ok(self.raw_metadata().await?.into())
    }

    pub async fn raw_metadata(&self) -> Result<HashMap<String, MetadataValue>, MprisError> {
        let data = self.player_proxy.metadata().await?;
        let raw: HashMap<String, MetadataValue> =
            data.into_iter().map(|(k, v)| (k, v.into())).collect();
        Ok(raw)
    }

    pub fn bus_name(&self) -> &str {
        self.mp2_proxy.bus_name()
    }

    pub async fn quit(&self) -> Result<(), MprisError> {
        Ok(self.mp2_proxy.quit().await?)
    }

    pub async fn can_quit(&self) -> Result<bool, MprisError> {
        Ok(self.mp2_proxy.can_quit().await?)
    }

    pub async fn raise(&self) -> Result<(), MprisError> {
        Ok(self.mp2_proxy.raise().await?)
    }

    pub async fn can_raise(&self) -> Result<bool, MprisError> {
        Ok(self.mp2_proxy.can_raise().await?)
    }

    pub async fn desktop_entry(&self) -> Result<String, MprisError> {
        Ok(self.mp2_proxy.desktop_entry().await?)
    }

    pub async fn has_track_list(&self) -> Result<bool, MprisError> {
        Ok(self.mp2_proxy.has_track_list().await?)
    }

    pub async fn identity(&self) -> Result<String, MprisError> {
        Ok(self.mp2_proxy.identity().await?)
    }

    pub async fn supported_mime_types(&self) -> Result<Vec<String>, MprisError> {
        Ok(self.mp2_proxy.supported_mime_types().await?)
    }

    pub async fn supported_uri_schemes(&self) -> Result<Vec<String>, MprisError> {
        Ok(self.mp2_proxy.supported_uri_schemes().await?)
    }

    pub async fn can_control(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_control().await?)
    }

    pub async fn next(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.next().await?)
    }

    pub async fn can_go_next(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_go_next().await?)
    }

    pub async fn previous(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.previous().await?)
    }

    pub async fn can_go_previous(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_go_previous().await?)
    }

    pub async fn play(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.play().await?)
    }

    pub async fn can_play(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_play().await?)
    }

    pub async fn pause(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.pause().await?)
    }

    pub async fn can_pause(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_pause().await?)
    }

    pub async fn play_pause(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.play_pause().await?)
    }

    pub async fn stop(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.stop().await?)
    }

    pub async fn stop_after_current(&self) -> Result<(), MprisError> {
        Ok(self.player_proxy.stop_after_current().await?)
    }

    pub async fn seek(&self, offset_in_microseconds: i64) -> Result<(), MprisError> {
        Ok(self.player_proxy.seek(offset_in_microseconds).await?)
    }

    pub async fn can_seek(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.can_seek().await?)
    }

    pub async fn get_position(&self) -> Result<i64, MprisError> {
        Ok(self.player_proxy.position().await?)
    }

    pub async fn set_position(&self, track_id: &TrackID, position: i64) -> Result<(), MprisError> {
        Ok(self
            .player_proxy
            .set_position(&track_id.get_object_path(), position)
            .await?)
    }

    pub async fn get_loop_status(&self) -> Result<LoopStatus, MprisError> {
        Ok(self.player_proxy.loop_status().await?.parse()?)
    }

    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> Result<(), MprisError> {
        Ok(self
            .player_proxy
            .set_loop_status(loop_status.as_str())
            .await?)
    }

    pub async fn playback_status(&self) -> Result<PlaybackStatus, MprisError> {
        Ok(self.player_proxy.playback_status().await?.parse()?)
    }

    pub async fn open_uri(&self, uri: &str) -> Result<(), MprisError> {
        Ok(self.player_proxy.open_uri(uri).await?)
    }

    pub async fn maximum_rate(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.maximum_rate().await?)
    }

    pub async fn minimum_rate(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.minimum_rate().await?)
    }

    pub async fn get_playback_rate(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.rate().await?)
    }

    pub async fn set_playback_rate(&self, rate: f64) -> Result<(), MprisError> {
        Ok(self.player_proxy.set_rate(rate).await?)
    }

    pub async fn get_shuffle(&self) -> Result<bool, MprisError> {
        Ok(self.player_proxy.shuffle().await?)
    }

    pub async fn set_shuffle(&self, shuffle: bool) -> Result<(), MprisError> {
        Ok(self.player_proxy.set_shuffle(shuffle).await?)
    }

    pub async fn get_volume(&self) -> Result<f64, MprisError> {
        Ok(self.player_proxy.volume().await?)
    }

    pub async fn set_volume(&self, volume: f64) -> Result<(), MprisError> {
        Ok(self.player_proxy.set_volume(volume).await?)
    }
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
            .field("bus_name", &self.bus_name())
            .finish()
    }
}
